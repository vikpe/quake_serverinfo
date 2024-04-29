# quake_serverinfo [![Test](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_serverinfo)](https://crates.io/crates/quake_serverinfo) [![docs.rs](https://img.shields.io/docsrs/quake_serverinfo)](https://docs.rs/quake_serverinfo/)

> A crate for handling QuakeWorld serverinfo strings

## Usage

```rust
let info_str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\maxspectators\12\*admin\alpha <alpha@foo.com>\ktxver\1.42\sv_antilag\2\needpass\4\*gamedir\qw\mode\1on1\*qvm\so\*progs\so\maxclients\2\timelimit\10\deathmatch\3\map\aerowalk"#;
let info = Serverinfo::from_str(&info_str);

println!("{:?}", info.maxfps);  // Some(77)
println!("{:?}", info.version); // Some("MVDSV 0.36")
```
