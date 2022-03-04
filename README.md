# unfair-advantage

[Fat Pixels font](https://www.1001fonts.com/fat-pixels-font.html)

# Launching

```
$ cargo install wasm-server-runner
$ cargo run-wasm
```

# Building

Make sure you have installed
```
$ cargo install -f wasm-bindgen-cli
```

In order to build it use `build.sh` script

note: if it fails remove `bevy/dynamic` feature from cargo.toml

```
default = ["bevy/dynamic", "bevy/wayland"]
```

to serve it you can use npm
```
npx serve .
```