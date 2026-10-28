use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub uuid: String,
    pub name: String,
    pub token: String,
    pub quota_bytes: u64,
    pub expiry_timestamp: u64,
    pub enabled: bool,
}

pub struct UserState {
    pub config: UserConfig,
    pub used_bytes: AtomicU64,
}

#[derive(Clone)]
pub struct UserManager {
    users: Arc<RwLock<HashMap<String, Arc<UserState>>>>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_user(&self, config: UserConfig) {
        let mut map = self.users.write().await;
        let state = Arc::new(UserState {
            config: config.clone(),
            used_bytes: AtomicU64::new(0),
        });
        map.insert(config.uuid, state);
    }

    pub async fn authenticate(&self, uuid: &str, token: &str) -> Option<Arc<UserState>> {
        let map = self.users.read().await;
        if let Some(user) = map.get(uuid) {
            if user.config.enabled && user.config.token == token {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                if user.config.expiry_timestamp > 0 && current_time > user.config.expiry_timestamp {
                    return None;
                }

                if user.config.quota_bytes > 0 {
                    let used = user.used_bytes.load(Ordering::Relaxed);
                    if used >= user.config.quota_bytes {
                        return None;
                    }
                }

                return Some(user.clone());
            }
        }
        None
    }

    pub fn record_traffic(user: &UserState, bytes: u64) {
        user.used_bytes.fetch_add(bytes, Ordering::Relaxed);
    }
}