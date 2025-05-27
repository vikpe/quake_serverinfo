# quake_serverinfo [![Test](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vikpe/quake_serverinfo/graph/badge.svg?token=KwNnQ0ICcS)](https://codecov.io/gh/vikpe/quake_serverinfo) [![crates.io](https://img.shields.io/crates/v/quake_serverinfo)](https://crates.io/crates/quake_serverinfo) [![docs.rs](https://img.shields.io/docsrs/quake_serverinfo)](https://docs.rs/quake_serverinfo/)

> A Rust crate for parsing QuakeWorld serverinfo strings

## Usage

```rust
use quake_serverinfo::Settings;

let settings = Settings::from(r#"\maxfps\77\matchtag\kombat"#);
assert_eq!(settings.maxfps, Some(77));
assert_eq!(settings.matchtag, Some("kombat".to_string()));
```

## Fields

```rust
pub struct Settings {
    pub admin: Option<String>,
    pub broadcast: Option<i32>,
    pub city: Option<String>,
    pub coords: Option<String>,
    pub countrycode: Option<String>,
    pub deathmatch: Option<i32>,
    pub epoch: Option<i32>,
    pub fpd: Option<i32>,
    pub fraglimit: Option<i32>,
    pub gamedir: Option<String>,
    pub hostname: Option<String>,
    pub hostport: Option<String>,
    pub ktxmode: Option<String>,
    pub ktxver: Option<String>,
    pub map: Option<String>,
    pub matchtag: Option<String>,
    pub maxclients: Option<i32>,
    pub maxfps: Option<i32>,
    pub maxspectators: Option<i32>,
    pub mode: Option<String>,
    pub needpass: Option<i32>,
    pub pm_ktjump: Option<i32>,
    pub progs: Option<String>,
    pub qvm: Option<String>,
    pub serverdemo: Option<String>,
    pub status: Option<String>,
    pub sv_antilag: Option<i32>,
    pub teamplay: Option<i32>,
    pub timelimit: Option<i32>,
    pub version: Option<String>,
    pub z_ext: Option<i32>,
}
```

## See also

- [quake_infostring](https://github.com/vikpe/quake_infostring) - A Rust crate for parsing generic QuakeWorld info strings
