// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use std::path::PathBuf;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QList, QString, QStringList};

use std::fs;

use crate::qstandardpaths::{
    QStandardPaths, QStandardPathsLocateOption, QStandardPathsStandardLocation,
};

#[cxx_qt::bridge()]
mod ffi {
    unsafe extern "C++" {
        include!("KFileUtils");

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        #[namespace = "KFileUtils"]
        #[rust_name = "find_all_unique_files"]
        pub fn findAllUniqueFiles(dirs: &QStringList, nameFilters: &QStringList) -> QStringList;
    }
}

fn kcfg_dirs(location: &QString) -> QStringList {
    if location.is_empty() {
        return QStandardPaths::locate_all(
            QStandardPathsStandardLocation::GenericDataLocation,
            &QString::from("config.kcfg"),
            QStandardPathsLocateOption::LocateDirectory.into(),
        );
    } else {
        let mut path = PathBuf::new();
        path.push(&location.to_string());
        path.push("share");
        path.push("config.kcfg");

        return QStringList::from(&QString::from(path.to_str().unwrap()));
    }
}

use ffi::find_all_unique_files;

pub fn find_kcfg_files(location: &QString) -> QStringList {
    let mut patterns = QStringList::default();
    patterns.append(QString::from("*.kcfg"));

    let dirs = kcfg_dirs(location);

    return find_all_unique_files(&dirs, &patterns);
}
