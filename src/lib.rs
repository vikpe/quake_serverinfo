//! # quake_serverinfo
//!
//! A parser for QuakeWorld server info strings, converting into a typed [`Settings`] struct.
//!
//! This crate can optionally derive serialization and deserialization using `serde`.
//!
//! # Examples
//! ```
//! use quake_serverinfo::Settings;
//!
//! let settings = Settings::from(r#"\maxfps\77\matchtag\kombat"#);
//! assert_eq!(settings.maxfps, Some(77));
//! assert_eq!(settings.matchtag, Some("kombat".to_string()));
//! ```

use std::collections::HashMap;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents parsed server info.
///
/// Each field is an optional setting parsed from a raw info string or map,
/// matching known keys used in server metadata.
///
/// # Examples
/// ```
/// use quake_serverinfo::Settings;
///
/// let settings = Settings::from(r#"\maxfps\77\matchtag\kombat"#);
/// assert_eq!(settings.maxfps, Some(77));
/// assert_eq!(settings.matchtag, Some("kombat".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&HashMap<String, String>> for Settings {
    fn from(value: &HashMap<String, String>) -> Self {
        Self {
            admin: map_get_string(value, "*admin"),
            broadcast: map_get_i32(value, "broadcast"),
            city: map_get_string(value, "city"),
            coords: map_get_string(value, "coords"),
            countrycode: map_get_string(value, "countrycode"),
            deathmatch: map_get_i32(value, "deathmatch"),
            epoch: map_get_i32(value, "epoch"),
            fpd: map_get_i32(value, "fpd"),
            fraglimit: map_get_i32(value, "fraglimit"),
            gamedir: map_get_string(value, "*gamedir"),
            hostname: map_get_string(value, "hostname"),
            hostport: map_get_string(value, "hostport"),
            ktxmode: map_get_string(value, "ktxmode"),
            ktxver: map_get_string(value, "ktxver"),
            map: map_get_string(value, "map"),
            matchtag: map_get_string(value, "matchtag"),
            maxclients: map_get_i32(value, "maxclients"),
            maxfps: map_get_i32(value, "maxfps"),
            maxspectators: map_get_i32(value, "maxspectators"),
            mode: map_get_string(value, "mode"),
            needpass: map_get_i32(value, "needpass"),
            pm_ktjump: map_get_i32(value, "pm_ktjump"),
            progs: map_get_string(value, "*progs"),
            qvm: map_get_string(value, "*qvm"),
            serverdemo: map_get_string(value, "serverdemo"),
            status: map_get_string(value, "status"),
            sv_antilag: map_get_i32(value, "sv_antilag"),
            teamplay: map_get_i32(value, "teamplay"),
            timelimit: map_get_i32(value, "timelimit"),
            version: map_get_string(value, "*version"),
            z_ext: map_get_i32(value, "*z_ext"),
        }
    }
}

impl From<&str> for Settings {
    fn from(value: &str) -> Self {
        Self::from(&quake_infostring::parse_key_values(value))
    }
}

impl From<&String> for Settings {
    fn from(value: &String) -> Self {
        Settings::from(value.as_str())
    }
}

impl From<String> for Settings {
    fn from(value: String) -> Self {
        Settings::from(value.as_str())
    }
}

impl From<&[u8]> for Settings {
    fn from(bytes: &[u8]) -> Self {
        Self::from(quake_text::bytestr::to_unicode(bytes).as_str())
    }
}

fn map_get_string(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).cloned()
}

fn map_get_i32(map: &HashMap<String, String>, key: &str) -> Option<i32> {
    map.get(key)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INFO_STR: &str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\hostname\QUAKE.SE KTX:28501\status\Standby\teamplay\2\*gamedir\qw\fpd\206\mode\2on2\*qvm\so\*progs\so\maxclients\4\timelimit\10\deathmatch\3\map\maphub_v1\broadcast\1"#;

    #[test]
    fn test_display() {
        let settings = Settings::from(INFO_STR);
        assert_eq!(
            settings.to_string(),
            r#"Settings { admin: Some("suom1 <suom1@irc.ax>"), broadcast: Some(1), city: None, coords: None, countrycode: None, deathmatch: Some(3), epoch: None, fpd: Some(206), fraglimit: None, gamedir: Some("qw"), hostname: Some("QUAKE.SE KTX:28501"), hostport: None, ktxmode: None, ktxver: Some("1.42"), map: Some("maphub_v1"), matchtag: None, maxclients: Some(4), maxfps: Some(77), maxspectators: Some(12), mode: Some("2on2"), needpass: None, pm_ktjump: Some(1), progs: Some("so"), qvm: Some("so"), serverdemo: None, status: Some("Standby"), sv_antilag: Some(2), teamplay: Some(2), timelimit: Some(10), version: Some("MVDSV 0.36"), z_ext: Some(511) }"#
        );
    }

    #[test]
    fn test_from_hashmap() {
        let map = HashMap::from([
            ("maxfps".to_string(), "77".to_string()),
            ("map".to_string(), "dm2".to_string()),
        ]);

        assert_eq!(
            Settings::from(&map),
            Settings {
                maxfps: Some(77),
                map: Some("dm2".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Settings::from(INFO_STR),
            Settings {
                admin: Some("suom1 <suom1@irc.ax>".to_string()),
                broadcast: Some(1),
                deathmatch: Some(3),
                fpd: Some(206),
                gamedir: Some("qw".to_string()),
                hostname: Some("QUAKE.SE KTX:28501".to_string()),
                ktxver: Some("1.42".to_string()),
                map: Some("maphub_v1".to_string()),
                maxclients: Some(4),
                maxfps: Some(77),
                maxspectators: Some(12),
                mode: Some("2on2".to_string()),
                pm_ktjump: Some(1),
                progs: Some("so".to_string()),
                qvm: Some("so".to_string()),
                status: Some("Standby".to_string()),
                sv_antilag: Some(2),
                teamplay: Some(2),
                timelimit: Some(10),
                version: Some("MVDSV 0.36".to_string()),
                z_ext: Some(511),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_from_string_ref() {
        assert_eq!(
            Settings::from(&INFO_STR.to_string()),
            Settings::from(INFO_STR)
        )
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            Settings::from(INFO_STR.to_string()),
            Settings::from(INFO_STR)
        )
    }

    #[test]
    fn test_from_bytes() {
        let bytes = INFO_STR.as_bytes();
        let settings = Settings::from(bytes);
        assert_eq!(settings.admin, Some("suom1 <suom1@irc.ax>".to_string()));
    }
}
