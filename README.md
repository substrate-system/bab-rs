# Bab

A Rust implementation of the [Bab](https://worm-blossom.github.io/bab/) family
of hash functions in general, and the
[WILLIAM3](https://worm-blossom.github.io/bab/#instantiations_william)
instantiation in particular.

See the [docs](https://docs.rs/bab_rs/latest) for more details.

<details><summary><h2>Contents</h2></summary>

<!-- toc -->

- [Fork](#fork)
  * [Example](#example)
- [WASM](#wasm)
  * [Prerequisites](#prerequisites)
  * [Compile](#compile)
- [bump the version and publish](#bump-the-version-and-publish)
  * [Bump the version](#bump-the-version)
  * [Install cargo-edit (first time only)](#install-cargo-edit-first-time-only)
  * [Publish](#publish)
  * [Build the WASM package](#build-the-wasm-package)
  * [Navigate to the generated package directory](#navigate-to-the-generated-package-directory)
  * [publish](#publish)
- [Test](#test)
- [Publish to crates.io](#publish-to-cratesio)
- [Consume](#consume)

<!-- tocstop -->

</details>

## Fork

This is a fork of [worm-blossom/bab_rs](https://codeberg.org/worm-blossom/bab_rs).

This fork builds a web assembly version of the wormblossom rust library.

### Example

```js
import init, {
    william3_hash,
    William3HasherWasm,
    william3_width
} from '@substrate-system/bab'

await init()

const input = 'hello bab'
const encoder = new TextEncoder()
const data = encoder.encode(input)
const hash = william3_hash(data)

const hasher = new William3HasherWasm()
hasher.write(encoder.encode('hello'))
const hash2 = hasher.finish_hex();

console.log(hash2)
// 5d70555767754cbd...
```

Start [the example page](./example/) with a localhost `vite` server:

```sh
npm start
```

## WASM

Compile the Rust code to web assembly.

### Prerequisites

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

### Compile

```sh
npm run build
```

This will create a `pkg/` directory containing the compiled WebAssembly module.


-------


## bump the version and publish

`Cargo.tonl` is the source of truth for the version, because this is primarily
a rust package.

### Bump the version

There's a cargo plugin `cargo-edit` that makes this easier.

### Install cargo-edit (first time only)

```sh
cargo install cargo-edit
```

#### Bump patch version (0.4.3 → 0.4.4)

```sh
cargo set-version --bump patch
```

#### Bump minor version (0.4.3 → 0.5.0)

```sh
cargo set-version --bump minor
```

#### Bump major version (0.4.3 → 1.0.0)

```sh
cargo set-version --bump major
```

#### Set specific version

```sh
cargo set-version 0.5.0
```


### Publish

Publish the package to `npm`.

### Build the WASM package

Frist build the package. It will use the version number from `Cargo.toml`.

```sh
./build-wasm.sh
```

### Navigate to the generated package directory

```sh
cd pkg
```

### publish

```sh
npm publish
```


-------


## Test

```sh
cargo test --features std
```

## Publish to crates.io

```sh
cargo publish
```


----


## Consume

#### In a Web Browser

See the file [./example/index.ts](./example/index.ts) for an example of
using this module.

```js
import init, {
    william3_hash,
    William3HasherWasm,
    william3_width
} from '@substrate-system/bab'
```
