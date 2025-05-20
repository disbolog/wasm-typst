# Development Environment
Use flake to enter the devShell: `nix develop`.

If using direnv: `direnv allow`.

# Testing and Compiling
`cargo test` to run tests.

`cargo build --release --target wasm32-unknown-unknown` and `typst compile example.typ`.

Will most likely error with:
```sh
error: cannot find definition for import __wbindgen_placeholder__::__wbindgen_describe with type Func(FuncType { params: [I32], results: [] })
  ┌─ example.typ:3:31
  │
3 │   let timeline_plugin = plugin("./target/wasm32-unknown-unknown/release/gantt_purely_rust.wasm")
  │                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
