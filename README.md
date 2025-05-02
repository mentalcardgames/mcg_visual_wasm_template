## About

This template is designed for compiling Rust libraries into WebAssembly and
getting to know how you could use the [`mcg_visual`][visual-git] library crate.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[visual-git]: https://github.com/gSys1337/mcg_visual

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

TODO: Replace with proper/final git address

```shell
cargo generate --git https://github.com/gSys1337/wasm-mcg-visual-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```shell
wasm-pack build
```

and if you want faster build times I recommend disabling optimizations.

```shell
wasm-pack build --target web --no-opt
```

### 🧑‍🍳 Run server

You need a server to load your card images and all other assets if you don't want to
include them in the final binary.

```shell
python -m http.server 8080
```



### 🔬 Test in Headless Browsers with `wasm-pack test`

TODO: Think if tests should be included in this template.
If yes then get to know how to properly include them.

```shell
wasm-pack test --headless --firefox
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you
* You can read [here](https://rustwasm.github.io/docs/book/) through an example/a tutorial
on how work at a `rust+wasm` looks like. 

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
