rust-demo-use
==

Demo how to use refer to a github package.

## Usage 

```sh
make all
```

```sh
make rub
```

```sh
cargo run
```

## dependency

### Cargo.toml

```toml
[dependencies]
rust-lib-01 = { git = "https://github.com/mitchallen/rust-lib-01.git", tag = "v0.1.2" }
```

## code

Example usage:

```rs
let x = rust_lib_01::add(a, b);
```
