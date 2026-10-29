/**
 * QCore Edge Worker - WebAssembly Transport Bridge
 *
 * Imports the Rust core via WebAssembly and bridges client WebSockets
 * directly to outbound TCP sockets via the Cloudflare Sockets API.
 */
import { connect } from 'cloudflare:sockets';
import init, { verify_and_decrypt_handshake } from '@qcore-wasm';
import wasmBinary from '@qcore-wasm-bg';

const DEFAULT_FALLBACK_DOMAIN = 'swdist.apple.com';
const DEFAULT_AUTH_TOKEN = 'qcore-secure-token';

let isWasmLoaded = false;

async function ensureWasm() {
  if (!isWasmLoaded) {
    await init(wasmBinary);
    isWasmLoaded = true;
  }
}

export default {
  async fetch(request, env, ctx) {
    const upgradeHeader = request.headers.get('Upgrade');
    if (upgradeHeader && upgradeHeader.toLowerCase() === 'websocket') {
      return handleSession(request, env, ctx);
    }
    return handleFallback(request, env);
  }
};

async function handleSession(request, env, ctx) {
  try {
    await ensureWasm();
  } catch (err) {
    return new Response(`WebAssembly runtime initialization error: ${err.message}`, { status: 500 });
  }

  const pair = new WebSocketPair();
  const [clientWs, serverWs] = Object.values(pair);
  serverWs.accept();

  const classicSecret = env.CLASSIC_SECRET_B64 || '';
  const pqSecret = env.PQ_DECAPS_KEY_B64 || '';
  const token = env.AUTH_TOKEN || DEFAULT_AUTH_TOKEN;

  let handshakeDone = false;
  let tcp = null;
  let writer = null;
  let closed = false;

  function closeAll() {
    if (closed) return;
    closed = true;
    try { if (writer) writer.releaseLock(); } catch (_) {}
    try { if (tcp) tcp.close(); } catch (_) {}
    try { if (serverWs.readyState === WebSocket.OPEN) serverWs.close(1000, 'Closed'); } catch (_) {}
  }

  serverWs.addEventListener('message', async (e) => {
    try {
      if (!handshakeDone) {
        const raw = new Uint8Array(e.data);
        const res = verify_and_decrypt_handshake(raw, classicSecret, pqSecret, token);
        if (!res.success) {
          serverWs.close(1008, `Handshake rejected: ${res.error}`);
          closeAll();
          return;
        }

        try {
          tcp = connect({ hostname: res.host, port: res.port });
          writer = tcp.writable.getWriter();
        } catch (err) {
          serverWs.close(1011, `Connection failed: ${err.message}`);
          closeAll();
          return;
        }

        handshakeDone = true;
        serverWs.send(new Uint8Array([0x00]));

        ctx.waitUntil((async () => {
          const reader = tcp.readable.getReader();
          try {
            while (true) {
              const { value, done } = await reader.read();
              if (done) break;
              if (serverWs.readyState === WebSocket.OPEN) {
                serverWs.send(value);
              } else {
                break;
              }
            }
          } catch (_) {}
          closeAll();
        })());
        return;
      }

      if (writer && e.data) {
        const chunk = e.data instanceof ArrayBuffer ? new Uint8Array(e.data) : new Uint8Array(e.data.buffer);
        await writer.write(chunk);
      }
    } catch (_) {
      closeAll();
    }
  });

  serverWs.addEventListener('close', closeAll);
  serverWs.addEventListener('error', closeAll);

  return new Response(null, { status: 101, webSocket: clientWs });
}

async function handleFallback(request, env) {
  const host = env.FALLBACK_DOMAIN || DEFAULT_FALLBACK_DOMAIN;
  const target = new URL(request.url);
  target.hostname = host;
  target.protocol = 'https:';
  target.port = '443';

  const headers = new Headers(request.headers);
  headers.set('Host', host);
  headers.set('Referer', `https://${host}/`);

  try {
    return await fetch(target.toString(), {
      method: request.method,
      headers: headers,
      body: request.method !== 'GET' && request.method !== 'HEAD' ? request.body : undefined,
      redirect: 'follow',
    });
  } catch (_) {
    return new Response('<!DOCTYPE html><html><body><h1>Edge Gateway Active</h1></body></html>', {
      status: 200,
      headers: { 'Content-Type': 'text/html; charset=utf-8' },
    });
  }
}