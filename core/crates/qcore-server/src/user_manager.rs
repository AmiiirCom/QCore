//! Multi-Tenant User Management & Traffic Accounting.
//!
//! Maintains active client credentials, expiration thresholds, and thread-safe
//! atomic byte counters for quota enforcement.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration record for an individual user account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserConfig {
    /// Unique user identifier (UUID v4).
    pub uuid: String,
    /// Friendly user display name.
    pub name: String,
    /// Cryptographic session authentication token.
    pub token: String,
    /// Bandwidth quota limit in bytes (0 represents unlimited).
    pub quota_bytes: u64,
    /// Unix timestamp expiration threshold (0 represents indefinite).
    pub expiry_timestamp: u64,
    /// Account administrative status flag.
    pub enabled: bool,
}

/// Runtime state container tracking live bandwidth usage.
pub struct UserState {
    /// Static user configuration parameters.
    pub config: UserConfig,
    /// Cumulative transferred bytes counter.
    pub used_bytes: AtomicU64,
}

/// Thread-safe in-memory manager for multi-tenant authentication.
#[derive(Clone)]
pub struct UserManager {
    users: Arc<RwLock<HashMap<String, Arc<UserState>>>>,
}

impl UserManager {
    /// Constructs a new empty user manager.
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a user configuration into the active state.
    pub async fn add_user(&self, config: UserConfig) {
        let mut map = self.users.write().await;
        let state = Arc::new(UserState {
            config: config.clone(),
            used_bytes: AtomicU64::new(0),
        });
        map.insert(config.uuid, state);
    }

    /// Synchronizes in-memory users with updated configuration.
    /// Preserves accumulated byte metrics for existing users.
    pub async fn sync_users(&self, updated_users: Vec<UserConfig>) {
        let mut map = self.users.write().await;
        let mut active_uuids = HashSet::new();

        for u in updated_users {
            active_uuids.insert(u.uuid.clone());
            if let Some(existing) = map.get(&u.uuid) {
                let used = existing.used_bytes.load(Ordering::Relaxed);
                map.insert(
                    u.uuid.clone(),
                    Arc::new(UserState {
                        config: u,
                        used_bytes: AtomicU64::new(used),
                    }),
                );
            } else {
                map.insert(
                    u.uuid.clone(),
                    Arc::new(UserState {
                        config: u,
                        used_bytes: AtomicU64::new(0),
                    }),
                );
            }
        }

        // Drop revoked accounts no longer present in the updated configuration file
        map.retain(|uuid, _| active_uuids.contains(uuid));
    }

    /// Authenticates a client connection and evaluates quota/expiry policies.
    pub async fn authenticate(&self, uuid: &str, token: &str) -> Option<Arc<UserState>> {
        let map = self.users.read().await;
        if let Some(user) = map.get(uuid) {
            if user.config.enabled && user.config.token == token {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                // Expiration evaluation
                if user.config.expiry_timestamp > 0 && current_time > user.config.expiry_timestamp {
                    return None;
                }

                // Bandwidth quota evaluation
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

    /// Atomically increments the transferred byte counter for a user.
    pub fn record_traffic(user: &UserState, bytes: u64) {
        user.used_bytes.fetch_add(bytes, Ordering::Relaxed);
    }
}