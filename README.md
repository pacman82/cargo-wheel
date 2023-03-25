# cargo-wheel

## About

Use milksnake and cbindgen to generate python binding to your Rust crate.

**Deprecated**

milksnake has not seen many updates in the last year and seems to have been replaced by maturin. I would not recommend using this crate anymore.

## Quick start

Edit your `Cargo.toml` and set the crate type to `cdylib`

```toml
[lib]
crate-type = ["cdylib"]
```

Export functions or datastructures in your library to make them visible to `C`.

```rust
#[no_mangle]
pub extern fn greet() {
    println!("Hello from Rust");
}
```

Call `cargo wheel` to invoke cbindgen and set up a python package.

```bash
cargo wheel
```

Use `cffi` in the generated `__init__.py` to expose the functionality to python

```python
from test_lib._native import ffi, lib

def greet():
    lib.greet()
```

## Why you want to use cargo-wheel

To save boilerplate if creating python bindings for a Rust crate

## Why you do not want to use cargo wheel

Scenarios where you want to invoke `cargo` from your `setup.py` rather than the other way around.
E.g. If you want to replace python code with Rust in an existing wheel.