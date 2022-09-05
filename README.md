# goldsrc-rs

Collection of file's parsers for goldsrc engine.

# Supported files

- [x] **.wad** containing fonts, mip textures, simple pictures
- [x] **.bsp** with all lumps support
- [ ] **.spr**
- [ ] **.mdl**

## Installation

In your **Cargo.toml** add new dependency:

```toml
[dependcies]
goldsrc-rs = "0.8"
```

## Usage

```rust
fn main() {
    let file = File::open("test.wad").unwrap();
    let wad = goldsrc_rs::wad(file);
    // ...

    let file = File::open("test.bsp").unwrap();
    let bsp = goldsrc_rs::bsp(file);

    // ...
}
```

## Contributing

Pull requests are welcome. This hasn't been tested appropriately.

So I'll be glad to see your results and bugs.

## License

[MIT](https://choosealicense.com/licenses/mit/)