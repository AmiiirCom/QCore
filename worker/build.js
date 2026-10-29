const fs = require('fs');
const path = require('path');
const esbuild = require('esbuild');
const JavaScriptObfuscator = require('javascript-obfuscator');

async function build() {
  console.log('[*] Preparing QCore Worker build pipeline...');

  // 1. Verify that the Rust WebAssembly core has been compiled
  const wasmPkgDir = path.resolve(__dirname, '../core/crates/qcore-wasm/pkg');
  const wasmBinaryPath = path.resolve(wasmPkgDir, 'qcore_wasm_bg.wasm');
  const wasmJsPath = path.resolve(wasmPkgDir, 'qcore_wasm.js');

  if (!fs.existsSync(wasmBinaryPath) || !fs.existsSync(wasmJsPath)) {
    console.error('[-] Error: WebAssembly artifacts not found!');
    console.error('[-] Please run the following command first:');
    console.error('    cd core/crates/qcore-wasm && wasm-pack build --target web --release');
    process.exit(1);
  }

  // 2. Resolve entrypoint (supporting both worker/src/index.js and worker/index.js)
  let entryPoint = path.resolve(__dirname, 'src/index.js');
  if (!fs.existsSync(entryPoint)) {
    const rootEntry = path.resolve(__dirname, 'index.js');
    if (fs.existsSync(rootEntry)) {
      entryPoint = rootEntry;
    } else {
      // Auto-create worker/src directory if missing
      const srcDir = path.resolve(__dirname, 'src');
      if (!fs.existsSync(srcDir)) {
        fs.mkdirSync(srcDir, { recursive: true });
      }
      console.error(`[-] Error: Worker entrypoint not found at ${entryPoint} or ${rootEntry}`);
      process.exit(1);
    }
  }

  console.log(`[*] Using entrypoint: ${entryPoint}`);
  console.log('[*] Inlining WebAssembly bytecode and bundling via esbuild...');

  const distDir = path.resolve(__dirname, 'dist');
  if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir, { recursive: true });
  }

  const tempBundle = path.resolve(distDir, 'bundle.tmp.js');
  const finalOutput = path.resolve(distDir, 'worker.js');

  // 3. Bundle everything into a single standalone ES module with inlined WASM
  await esbuild.build({
    entryPoints: [entryPoint],
    bundle: true,
    format: 'esm',
    target: 'es2022',
    outfile: tempBundle,
    external: ['cloudflare:sockets'],
    loader: {
      '.wasm': 'binary',
    },
    alias: {
      '@qcore-wasm': wasmJsPath,
      '@qcore-wasm-bg': wasmBinaryPath,
    },
  });

  console.log('[*] Applying high-strength JavaScript obfuscation...');
  const bundledCode = fs.readFileSync(tempBundle, 'utf8');

  // 4. Heavy obfuscation (control flow flattening, string encoding, etc.)
  const obfuscationResult = JavaScriptObfuscator.obfuscate(bundledCode, {
    compact: true,
    controlFlowFlattening: true,
    controlFlowFlatteningThreshold: 0.8,
    deadCodeInjection: true,
    deadCodeInjectionThreshold: 0.4,
    identifierNamesGenerator: 'hexadecimal',
    numbersToExpressions: true,
    simplify: true,
    splitStrings: true,
    splitStringsChunkLength: 8,
    stringArray: true,
    stringArrayEncoding: ['base64', 'rc4'],
    stringArrayThreshold: 0.85,
    transformObjectKeys: true,
    target: 'browser',
  });

  fs.writeFileSync(finalOutput, obfuscationResult.getObfuscatedCode(), 'utf8');

  // Clean temporary bundle
  if (fs.existsSync(tempBundle)) {
    fs.unlinkSync(tempBundle);
  }

  const sizeKb = (fs.statSync(finalOutput).size / 1024).toFixed(2);
  console.log(`[+] Success: Standalone obfuscated worker generated: ${finalOutput} (${sizeKb} KB)`);
}

build().catch((err) => {
  console.error('[-] Build failed:', err);
  process.exit(1);
});