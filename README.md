# quake_serverinfo [![Test](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_serverinfo/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_serverinfo)](https://crates.io/crates/quake_serverinfo) [![docs.rs](https://img.shields.io/docsrs/quake_serverinfo)](https://docs.rs/quake_serverinfo/)

> Parse QuakeWorld serverinfo strings

## Usage

```rust
let info_str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36"#;
let info = Serverinfo::from(&info_str);

println!("{:?}", info.maxfps);  // Some(77)
println!("{:?}", info.version); // Some("MVDSV 0.36")
println!("{:?}", info.admin);   // None
```


## Fields
```rust
pub struct Serverinfo {
    pub admin: Option<String>,
    pub deathmatch: Option<i32>,
    pub epoch: Option<i32>,
    pub fpd: Option<i32>,
    pub fraglimit: Option<i32>,
    pub gamedir: Option<String>,
    pub hostname: Option<String>,
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
    pub status: Option<String>,
    pub serverdemo: Option<String>,
    pub sv_antilag: Option<i32>,
    pub teamplay: Option<i32>,
    pub timelimit: Option<i32>,
    pub version: Option<String>,
    pub z_ext: Option<i32>,
}
```