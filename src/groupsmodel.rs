// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QList, QString, QStringList};
use groupsmodel::GroupsRoles;
use std::path::PathBuf;
use std::pin::Pin;

use std::fs;

use crate::config;
use crate::config::{Group, Kcfg};
use crate::qstandardpaths::{
    QStandardPaths, QStandardPathsLocateOption, QStandardPathsStandardLocation,
};
use crate::util;

#[cxx_qt::bridge]
mod groupsmodel {
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

    #[qenum(GroupsModel)]
    enum GroupsRoles {
        Name,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, file_name, cxx_name="fileName", READ, WRITE = set_file_name, NOTIFY = file_name_changed)]
        #[qproperty(QString, location, READ, WRITE = set_location, NOTIFY = location_changed)]
        #[base = QAbstractListModel]
        type GroupsModel = super::GroupsModelRust;

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

pub struct GroupsModelRust {
    groups: Vec<Group>,
    file_name: QString,
    location: QString,
}

impl Default for GroupsModelRust {
    fn default() -> Self {
        Self {
            groups: vec![],
            file_name: QString::from(""),
            location: QString::from(""),
        }
    }
}

use cxx_qt::CxxQtType;
use groupsmodel::*;

impl groupsmodel::GroupsModel {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.groups.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = GroupsRoles { repr: role };

        if let Some(group) = self.groups.get(index.row() as usize) {
            match role {
                GroupsRoles::Name => {
                    return QVariant::from(&QString::from(&group.name));
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(GroupsRoles::Name.repr, "name".into());
        hash
    }

    fn rebuild(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();

        let kcfg_files = util::find_kcfg_files(&self.location);

        let configs: Vec<_> = Upcast::<QList<QString>>::upcast(&kcfg_files)
            .iter()
            .map(|path| config::parse(path.to_string().as_str()))
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
