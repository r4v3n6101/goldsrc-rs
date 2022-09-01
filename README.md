# goldsrc-rs

Collection of file's parsers for goldsrc engine. Working with:

# Supported files

- [x] **.wad** containing fonts, mip textures, simple pictures
- [x] **.bsp** with all lumps support
- [ ] **.spr**

## Installation

In your **Cargo.toml** add new dependency:

```toml
[dependcies]
goldsrc = "0.7"
# or using nom, default-features=false for disabling `byteorder` code
goldsrc = { version = "0.7", default-features = false, features = ["nom"] }
```

## Usage

```rust
fn main() {
    let file = File::open("test.wad").unwrap();
    let wad = goldsrc_rs::wad(file); // or wad_from_bytes
    // ...

    let file = File::open("test.bsp").unwrap();
    let bsp = goldsrc_rs::bsp(file);

    // ...
}
```

## Contributing

Pull requests are welcome. This haven't already been tested appropriately.

So you're welcome!

## License

[MIT](https://choosealicense.com/licenses/mit/)