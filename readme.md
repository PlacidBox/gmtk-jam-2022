# "Roll and Dice"

An entry for the [GMTK Jam 2022](https://itch.io/jam/gmtk-jam-2022). This project's itch.io page
is at https://kategorybee.itch.io/gmtk-jam-2022.

The game can be played in a web browser at https://placidbox.github.io/gmtk-jam-2022/.

## Building

Install a recent version of rust, and `cargo run`. To check the web version:

```
cargo build --target wasm32-unknown-unknown --release
```
copy the wasm build output from `target/wasm32-unknown-unknown/release/game.wasm` to `docs`
```
cd docs
basic-http-server.exe .
```
