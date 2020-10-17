Install
=

## Using rustup

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/kei.arima/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory located at:

  /Users/kei.arima/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/kei.arima/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/kei.arima/.profile
  /Users/kei.arima/.bash_profile

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>
```

### Update Rust

```
rustup update
```

### Rust toolchain

```
$ ls -l ~/.cargo/bin

cargo
cargo-clippy
cargo-fmt
cargo-miri
clippy-driver
rls
rust-gdb
rust-lldb
rustc
rustdoc
rustfmt
rustup
```


Cargo package manger
=

- init project: `cargo new <project>`
- build project: `cargo build`
- run project: `cargo run`
- test project: `cargo test`
- build project document: `cargo doc`
- publish library to crates.io `cargo publish`
