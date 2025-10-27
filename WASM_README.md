# WebAssembly Build

Compile the Bab hash function to WASM.

## Prerequisites

Install `wasm-pack`:

```sh
cargo install wasm-pack
```

## Building

Run the build script:

```bash
./build-wasm.sh
```

This will create a `pkg/` directory containing the compiled WebAssembly module
and JavaScript bindings.

## Usage

### In a Web Browser

1. Build the WASM module using the script above
2. Open `wasm-example.html` in a web browser (you may need to serve it via a
   local HTTP server)

Using a local server:
```bash
python3 -m http.server 8000
# Then open http://localhost:8000/wasm-example.html
```

### In a JavaScript/TypeScript Project

After building, you can import and use the module:

```js
import init, { william3_hash, William3HasherWasm } from './pkg/bab_rs.js';

// Initialize the WASM module
await init();

// Batch hashing
const data = new TextEncoder().encode('hello world');
const hash = william3_hash(data);
console.log('Hash:', hash);

// Incremental hashing
const hasher = new William3HasherWasm();
hasher.write(new TextEncoder().encode('hello '));
hasher.write(new TextEncoder().encode('world'));
const incrementalHash = hasher.finish_hex();
console.log('Incremental hash:', incrementalHash);
```

## API

### Functions

- `william3_hash(data: Uint8Array): string` - Hash data and return hex string
- `william3_hash_bytes(data: Uint8Array): Uint8Array` - Hash data and return bytes
- `william3_width(): number` - Get the digest width in bytes (32)

### Classes

- `William3HasherWasm` - Incremental hasher
  - `constructor()` - Create a new hasher
  - `write(data: Uint8Array)` - Write data incrementally
  - `finish_hex(): string` - Finalize and get hash as hex string
  - `finish_bytes(): Uint8Array` - Finalize and get hash as bytes

## Build Targets

The default build script compiles for the `web` target. You can also build for
other targets:

```bash
# For Node.js
wasm-pack build --target nodejs --features wasm,william3

# For bundlers (webpack, etc.)
wasm-pack build --target bundler --features wasm,william3

# For no modules (browser with script tag)
wasm-pack build --target no-modules --features wasm,william3
```
