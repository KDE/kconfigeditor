// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "kcfg")]
pub struct Kcfg {
    pub kcfgfile: Option<KcfgFile>,
    pub group: Vec<Group>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "kcfgfile")]
pub struct KcfgFile {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@arg")]
    pub arg: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "group")]
pub struct Group {
    #[serde(rename = "@name")]
    pub name: String,
    pub entry: Option<Vec<Entry>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "entry")]
pub struct Entry {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@type")]
    pub the_type: Type,
    pub label: Option<Label>,
    pub default: Option<Vec<Default>>,
    #[serde(rename = "@key")]
    pub key: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Type {
    String,
    Int,
    UInt,
    Bool,
    Font,
    IntList,
    StringList,
    DateTime,
    Enum,
    PathList,
    Double,
    Path,
    Color,
    Rect,
    LongLong,
    Size,
    Point,
    Url,
    Password,
    ULongLong,
    RectF,
    SizeF,
    PointF,
    Time,
    UrlList,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(rename = "label")]
pub struct Label(String);

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "default")]
pub struct Default(pub String);

pub fn parse(path: &str) -> Option<Kcfg> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let result: Result<Kcfg, _> = serde_xml_rs::from_reader(reader);

    if result.is_err() {
        println!("failed to parse {}", path);
    }

    Some(result.unwrap())
}

impl Label {
    pub fn label(&self) -> &str {
        return &self.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let k = parse("/home/nico/kde/src/ktrip/src/ktripsettings.kcfg").unwrap();
        assert_eq!(k.kcfgfile.unwrap().name, Some(String::from("ktriprc")));

        assert_eq!(k.group.len(), 3);

        assert_eq!(k.group[0].name, "General");
        assert_eq!(k.group[0].entry.as_ref().unwrap().len(), 1);
        assert_eq!(
            k.group[0].entry.as_ref().unwrap()[0].name,
            Some(String::from("firstRun"))
        );
        assert_eq!(k.group[0].entry.as_ref().unwrap()[0].the_type, Type::Bool);
        assert_eq!(
            k.group[0].entry.as_ref().unwrap()[0].label,
            Some(Label(String::from("First run")))
        );
        assert_eq!(
            k.group[0].entry.as_ref().unwrap()[0].default,
            Some(vec![Default(String::from("true"))])
        );

        assert_eq!(k.group[1].entry.as_ref().unwrap()[0].label, None);
        assert_eq!(
            k.group[1].entry.as_ref().unwrap()[0].default,
            Some(vec![Default(String::from("lastUsed"))])
        );

        assert_eq!(k.group[2].name, "Backends");
        assert_eq!(k.group[2].entry.as_ref().unwrap().len(), 1);
        assert_eq!(
            k.group[2].entry.as_ref().unwrap()[0].name,
            Some(String::from("EnabledBackends"))
        );
        assert_eq!(
            k.group[2].entry.as_ref().unwrap()[0].the_type,
            Type::StringList
        );
        assert_eq!(
            k.group[2].entry.as_ref().unwrap()[0].label,
            Some(Label(String::from("Enabled backends")))
        );
        assert_eq!(
            k.group[2].entry.as_ref().unwrap()[0].default,
            Some(vec![Default(String::from(""))])
        );
    }
}
