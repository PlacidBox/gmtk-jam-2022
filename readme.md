
```
cargo build --target wasm32-unknown-unknown
basic-http-server.exe .
```

it'll load and run. seems to need frames output often to update loading status, but that
seems natural. probably need to output a frame between each asset load.

audio formats? urgh. .wav is big. m4a isn't supported on windows at least.

camera seems to work fine. dunno how controller support will work in web browser.
