# Rust Tutorial

## 1. Install Rust and Cargo
rustup : a command line tool for managing Rust versions and associated tools
```bash
# download rustup
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

$ rustc --version

# update to a newly released version
$ rustup update

# uninstall rustup and rust
$ rustup self uninstall
```

.rs : Rust source file extension
```bash
# compile rust code
rustc main.rs

# run code
./main
```

cargo : Rust's build system and package manager

cargo build : build with default mode debug
- debug : fast build but slow run
- release : slow build but fast run

```bash
# create new project
cargo new hello_cargo

# build project -> result in target/debug/
cargo build

# build project for release -> result in target/release/
cargo build --release

# build and run project
cargo run

# check source for compile
cargo check 
```
