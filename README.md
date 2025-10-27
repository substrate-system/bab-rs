# Bab in Rust

A Rust implementation of the [Bab](https://worm-blossom.github.io/bab/) family
of hash functions in general, and the
[WILLIAM3](https://worm-blossom.github.io/bab/#instantiations_william)
instantiation in particular.

See the [docs](https://docs.rs/bab_rs/latest) for more details.

## Fork

This is a fork of [worm-blossom/bab_rs](https://codeberg.org/worm-blossom/bab_rs).

### Example

State [the example page](./example/) with a localhost `vite` server.

```sh
npm start
```

## WASM

This repo can compile to web assembly.

```sh
./build-wasm.sh
```

This will create a `pkg/` directory containing the compiled WebAssembly module.
