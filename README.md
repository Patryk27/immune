# Immune

> In this game you will take control of the immune system. Capture lymph nodes & build your army to defend against the hordes of viruses.
>
> Be warned though, you stand at a disadvantage - the viruses will never stop coming!

[Play it online!](https://dzejkop.itch.io/immune)

Game made for https://itch.io/jam/bevy-jam-1 by https://github.com/Dzejkop, https://github.com/Ryneqq & https://github.com/Patryk27.

Warning: might contain peanuts & seriously rough Rust code!

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

# Assets

[Fat Pixels font](https://www.1001fonts.com/fat-pixels-font.html)

# License

Licensed under the MIT license.
