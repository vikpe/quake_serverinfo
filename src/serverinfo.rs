use std::collections::HashMap;
use std::fmt::Display;

use crate::str::to_hashmap;

#[derive(Debug, Default, Eq, PartialEq)]
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

impl Display for Serverinfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serverinfo {
    pub fn from_str(str: &str) -> Self {
        Self::from_hashmap(&to_hashmap(str))
    }

    fn from_hashmap(map: &HashMap<String, String>) -> Self {
        Self {
            admin: map_get_string(map, "*admin"),
            deathmatch: map_get_int(map, "deathmatch"),
            epoch: map_get_int(map, "epoch"),
            fpd: map_get_int(map, "fpd"),
            fraglimit: map_get_int(map, "fraglimit"),
            gamedir: map_get_string(map, "*gamedir"),
            hostname: map_get_string(map, "hostname"),
            ktxmode: map_get_string(map, "ktxmode"),
            ktxver: map_get_string(map, "ktxver"),
            map: map_get_string(map, "map"),
            matchtag: map_get_string(map, "matchtag"),
            maxclients: map_get_int(map, "maxclients"),
            maxfps: map_get_int(map, "maxfps"),
            maxspectators: map_get_int(map, "maxspectators"),
            mode: map_get_string(map, "mode"),
            needpass: map_get_int(map, "needpass"),
            pm_ktjump: map_get_int(map, "pm_ktjump"),
            progs: map_get_string(map, "*progs"),
            qvm: map_get_string(map, "*qvm"),
            status: map_get_string(map, "status"),
            serverdemo: map_get_string(map, "serverdemo"),
            sv_antilag: map_get_int(map, "sv_antilag"),
            teamplay: map_get_int(map, "teamplay"),
            timelimit: map_get_int(map, "timelimit"),
            version: map_get_string(map, "*version"),
            z_ext: map_get_int(map, "*z_ext"),
        }
    }
}

fn map_get_string(map: &HashMap<String, String>, key: &str) -> Option<String> {
    map.get(key).map(|v| v.to_string())
}

fn map_get_int(map: &HashMap<String, String>, key: &str) -> Option<i32> {
    map.get(key)?.parse().ok()
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INFO_STR: &str = r#"\maxfps\77\pm_ktjump\1\*version\MVDSV 0.36\*z_ext\511\*admin\suom1 <suom1@irc.ax>\ktxver\1.42\sv_antilag\2\maxspectators\12\hostname\QUAKE.SE KTX:28501\status\Standby\teamplay\2\*gamedir\qw\fpd\206\mode\2on2\*qvm\so\*progs\so\maxclients\4\timelimit\10\deathmatch\3\map\maphub_v1"#;

    #[test]
    fn test_display() {
        let info = Serverinfo::from_str(INFO_STR);
        assert_eq!(info.to_string(), r#"Serverinfo { admin: Some("suom1 <suom1@irc.ax>"), deathmatch: Some(3), epoch: None, fpd: Some(206), fraglimit: None, gamedir: Some("qw"), hostname: Some("QUAKE.SE KTX:28501"), ktxmode: None, ktxver: Some("1.42"), map: Some("maphub_v1"), matchtag: None, maxclients: Some(4), maxfps: Some(77), maxspectators: Some(12), mode: Some("2on2"), needpass: None, pm_ktjump: Some(1), progs: Some("so"), qvm: Some("so"), status: Some("Standby"), serverdemo: None, sv_antilag: Some(2), teamplay: Some(2), timelimit: Some(10), version: Some("MVDSV 0.36"), z_ext: Some(511) }"#);
    }

    #[test]
    fn test_from_str() {
        let info = Serverinfo::from_str(INFO_STR);
        assert_eq!(info.admin, Some("suom1 <suom1@irc.ax>".to_string()));
        assert_eq!(info.deathmatch, Some(3));
        assert_eq!(info.fpd, Some(206));
        assert_eq!(info.gamedir, Some("qw".to_string()));
        assert_eq!(info.hostname, Some("QUAKE.SE KTX:28501".to_string()));
        assert_eq!(info.ktxver, Some("1.42".to_string()));
        assert_eq!(info.map, Some("maphub_v1".to_string()));
        assert_eq!(info.maxclients, Some(4));
        assert_eq!(info.maxfps, Some(77));
        assert_eq!(info.maxspectators, Some(12));
        assert_eq!(info.mode, Some("2on2".to_string()));
        assert_eq!(info.pm_ktjump, Some(1));
        assert_eq!(info.progs, Some("so".to_string()));
        assert_eq!(info.qvm, Some("so".to_string()));
        assert_eq!(info.status, Some("Standby".to_string()));
        assert_eq!(info.sv_antilag, Some(2));
        assert_eq!(info.teamplay, Some(2));
        assert_eq!(info.timelimit, Some(10));
        assert_eq!(info.version, Some("MVDSV 0.36".to_string()));
        assert_eq!(info.z_ext, Some(511));
    }
}
