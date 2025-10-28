# Bab

[![Test](https://github.com/substrate-system/bab-rs/actions/workflows/test.yml/badge.svg)](https://github.com/substrate-system/bab-rs/actions/workflows/test.yml)
[![Deploy Demo](https://github.com/substrate-system/bab-rs/actions/workflows/deploy-demo.yml/badge.svg)](https://github.com/substrate-system/bab-rs/actions/workflows/deploy-demo.yml)

A Rust implementation of the [Bab](https://worm-blossom.github.io/bab/) family
of hash functions in general, and the
[WILLIAM3](https://worm-blossom.github.io/bab/#instantiations_william)
instantiation in particular.

[Try the live demo](https://substrate-system.github.io/bab-rs/)

See the [docs](https://docs.rs/bab_rs/latest) for more details
(this is the repo that I forked to create this package).

<details><summary><h2>Contents</h2></summary>

<!-- toc -->

- [Fork](#fork)
- [Install](#install)
- [Example](#example)
- [WASM](#wasm)
  * [Prerequisites](#prerequisites)
  * [Compile](#compile)
- [bump the version and publish](#bump-the-version-and-publish)
  * [Bump the version](#bump-the-version)
  * [Install cargo-edit (first time only)](#install-cargo-edit-first-time-only)
  * [Build and publish to `npm`](#build-and-publish-to-npm)
- [Test](#test)
- [Publish to crates.io](#publish-to-cratesio)
- [Consume](#consume)

<!-- tocstop -->

</details>

## Fork

This is a fork of [worm-blossom/bab_rs](https://codeberg.org/worm-blossom/bab_rs).

This fork builds a web assembly version of the wormblossom rust library.

## Install

This is published as an npm package.

```sh
npm i -S @substrate-system/bab
```

Installing via `npm` will give you access to the WASM files.

## Example

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

This project compiles Rust code to web assembly.

### Prerequisites

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

### Compile

```sh
npm run build
```

This npm script calls a local shell script, [build-was.sh](./build-wasm.sh),
which will create a `pkg/` directory and compile web assembly to it.
This `pkg` directory is what gets published.


-------


## bump the version and publish

`Cargo.toml` is the source of truth for the version, because this is primarily
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


### Build and publish to `npm`

Frist build the package. It will use the version number from `Cargo.toml`.

```sh
./build-wasm.sh
```

Then publish to `npm`.

```sh
cd pkg
```

```sh
npm publish
```


-------


## Test

The library is `no_std` by default, so tests require the `std` feature:

```sh
# Run tests
cargo test --features std

# Or use the convenient alias
cargo t

# Run all checks (format, clippy, tests)
cargo check-all
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
