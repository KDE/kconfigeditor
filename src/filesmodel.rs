// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cxx_qt_lib::QString;
use filesmodel::Roles;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;

use std::collections::HashSet;
use std::fs;

use crate::config;

#[cxx_qt::bridge]
mod filesmodel {
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

    #[qenum(FilesModel)]
    enum Roles {
        Name,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, location, READ, WRITE = set_location, NOTIFY = location_changed)]
        #[base = QAbstractListModel]
        type FilesModel = super::FilesModelRust;

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

        fn set_location(self: Pin<&mut Self>, location: QString);

        #[qsignal]
        #[cxx_name = "locationChanged"]
        fn location_changed(self: Pin<&mut Self>);
    }
}

pub struct FilesModelRust {
    apps: Vec<QString>,
    location: QString,
}

impl Default for FilesModelRust {
    fn default() -> Self {
        Self {
            apps: vec![],
            location: QString::from(""),
        }
    }
}

use cxx_qt::CxxQtType;
use filesmodel::*;

impl filesmodel::FilesModel {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.apps.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = Roles { repr: role };

        if let Some(name) = self.apps.get(index.row() as usize) {
            match role {
                Roles::Name => {
                    return name.into();
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(Roles::Name.repr, "name".into());
        hash
    }

    fn rebuild(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();

        let mut path = PathBuf::new();

        path.push(&self.location.to_string());
        path.push("share");
        path.push("config.kcfg");

        let paths = match fs::read_dir(path) {
            Ok(r) => r,
            Err(e) => {
                println!("error: {e:?}");
                return {};
            }
        };

        let configs: Vec<config::Kcfg> = paths
            .into_iter()
            .map(|path| config::parse(path.unwrap().path().to_str().unwrap()))
            .flatten()
            .collect();

        self.as_mut().rust_mut().apps = configs
            .iter()
            .map(|kcfg| kcfg.kcfgfile.clone())
            .flatten()
            .map(|file| file.name)
            .flatten()
            .collect::<HashSet<_>>()
            .iter()
            .map(|s| QString::from(s))
            .collect();

        self.end_reset_model();
    }

    pub fn set_location(mut self: Pin<&mut Self>, location: QString) {
        self.as_mut().rust_mut().location = location;
        self.as_mut().location_changed();

        self.rebuild();
    }
}
