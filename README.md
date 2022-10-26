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

 `4. Test!`
 
 To test contract, move to `contracts/astar_sns` and run commend below in terminal!

 ```
cargo +nightly-2022-08-15 test -- --nocapture
 ``` 

 The result looks just like this in terminal!
 ```
message_list_alice_bob: [Message { message: "Sorry Bro, I can't go today.", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:30" }, Message { message: "Why don't we go there tomorrow?", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:33" }, Message { message: "Hey, charlie will come!", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:35" }, Message { message: "He seems so muscular, so he would teach us.", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:36" }, Message { message: "Why don't we go there tomorrow?", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:37" }]

last_message_alice_bob: Message { message: "I'm so looking forward that!", sender_id: AccountId([2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), created_time: "12:38" }

message_list_alice_charlie: [Message { message: "Hey bro! Tomorrow I and Bob go to gym. Don't you join us?", sender_id: AccountId([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), created_time: "12:34" }]
test astar_sns::tests::test_message_fn_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
 ```

 There are a lot of tests so most of them are comment out. If you wanna see other test's result, you should just undo comment out of them and run commend upward!
