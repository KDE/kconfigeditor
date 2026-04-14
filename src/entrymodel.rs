// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cxx_qt_lib::QString;
use entrymodel::EntryRoles;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;

use std::collections::HashSet;
use std::fs;

use crate::config;
use crate::config::{Group, Kcfg};

#[cxx_qt::bridge]
mod entrymodel {
    unsafe extern "C++" {
        include!(< QAbstractListModel >);
        type QAbstractListModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
    }

    #[qenum(EntryModel)]
    enum EntryRoles {
        Name,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, file_name, cxx_name="fileName", READ, WRITE = set_file_name, NOTIFY = file_name_changed)]
        #[qproperty(QString, location, READ, WRITE = set_location, NOTIFY = location_changed)]
        #[base = QAbstractListModel]
        type EntryModel = super::EntryModelRust;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(&self, parent: &QModelIndex) -> i32;

        #[cxx_override]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(&self) -> QHash_i32_QByteArray;

        #[inherit]
        #[rust_name = "begin_reset_model"]
        fn beginResetModel(self: Pin<&mut Self>);

        #[inherit]
        #[rust_name = "end_reset_model"]
        fn endResetModel(self: Pin<&mut Self>);

        fn set_file_name(self: Pin<&mut Self>, file_name: QString);

        fn set_location(self: Pin<&mut Self>, location: QString);

        #[qsignal]
        #[cxx_name = "filenameChanged"]
        fn file_name_changed(self: Pin<&mut Self>);

        #[qsignal]
        #[cxx_name = "locationChanged"]
        fn location_changed(self: Pin<&mut Self>);
    }
}

pub struct EntryModelRust {
    groups: Vec<Group>,
    file_name: QString,
    location: QString,
}

impl Default for EntryModelRust {
    fn default() -> Self {
        Self {
            groups: vec![],
            file_name: QString::from(""),
            location: QString::from(""),
        }
    }
}

use cxx_qt::CxxQtType;
use entrymodel::*;

impl entrymodel::EntryModel {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.groups.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = EntryRoles { repr: role };

        if let Some(group) = self.groups.get(index.row() as usize) {
            match role {
                EntryRoles::Name => {
                    return QVariant::from(&QString::from(&group.name));
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(EntryRoles::Name.repr, "name".into());
        hash
    }

    fn rebuild(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();

        let mut path = PathBuf::new();
        path.push(&self.location.to_string());
        path.push("files");
        path.push("share");
        path.push("config.kcfg");

        let paths = match fs::read_dir(path) {
            Ok(r) => r,
            Err(e) => {
                println!("error: {e:?}");
                return {};
            }
        };

        let configs: Vec<Kcfg> = paths
        .into_iter()
        .map(|path| config::parse(path.unwrap().path().to_str().unwrap()))
        .flatten()
        .filter(|config| config.kcfgfile.as_ref().is_some())
        .filter(|config| {
            config
            .kcfgfile
            .as_ref()
            .unwrap()
            .name
            .clone()
            .unwrap_or_default()
            == self.file_name.to_string()
        })
        .collect();

        self.as_mut().rust_mut().groups = configs
        .iter()
        .flat_map(|config| config.group.clone())
        .collect();

        // TODO merge duplicate groups

        self.end_reset_model();
    }

    pub fn set_file_name(mut self: Pin<&mut Self>, file_name: QString) {
        self.as_mut().rust_mut().file_name = file_name;
        self.as_mut().file_name_changed();

        self.rebuild();
    }

    pub fn set_location(mut self: Pin<&mut Self>, location: QString) {
        self.as_mut().rust_mut().location = location;
        self.as_mut().location_changed();

        self.rebuild();
    }
}
