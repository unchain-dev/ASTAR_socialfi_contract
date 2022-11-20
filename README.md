### Set up your environment to run test🧪

To run test in your environment, run commend below in terminal.

`1. Enable Rust`

```
curl --proto '=https' --tlsv1.2 -sSf `https://sh.rustup.rs` | sh
```

`2. Set env variable`

```
source "$HOME/.cargo/env"
```

`3. Setup detailed setting`

```
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

rustup component add rust-src
(only Mac OS)brew install openssl
cargo install cargo-dylint dylint-link
cargo install --force --locked cargo-contract

rustup toolchain install nightly-2022-08-15

rustup target add wasm32-unknown-unknown --toolchain nightly-2022-08-15

rustup component add rust-src --toolchain nightly-2022-08-15
```

`4. Install ASTAR node`

Go to https://github.com/AstarNetwork/Astar/releases/tag/v4.24.0 and download the file `astar-collator-v4.24.0-[YOUR_OS]-x86_64.tar.gz`.

Then, expand the file using

```
tar xvf astar-collator-v4.24.0-[YOUR_OS]-x86
```

You can set up a local node using

```
./astar-collator --dev
```

You can view the block explorer for this local node [here](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).

`5. Compile contract`

Build the contract with

```
cargo +nightly-2022-08-15 contract build
```

`6. Test!`

To test the contract, first uncomment line 21 of `lib.rs`.
Then run commend below in terminal!

```
cargo +nightly-2022-08-15 test -- --nocapture
```

Make sure to comment this line out again whenever you make changes in your contract and need to recompile.
