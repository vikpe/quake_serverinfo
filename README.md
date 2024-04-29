# quake_serverinfo [![Test](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_serverinfo)](https://crates.io/crates/quake_serverinfo) [![docs.rs](https://img.shields.io/docsrs/quake_serverinfo)](https://docs.rs/quake_serverinfo/)

> Parse QuakeWorld serverinfo strings

## Usage

```rust
let info_str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36"#;
let info = Serverinfo::from_str(&info_str);

println!("{:?}", info.maxfps);  // Some(77)
println!("{:?}", info.version); // Some("MVDSV 0.36")
println!("{:?}", info.admin);   // None
```
