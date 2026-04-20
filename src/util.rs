// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use std::path::PathBuf;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QList, QString, QStringList};

use std::fs;

use crate::qstandardpaths::{
    QStandardPaths, QStandardPathsLocateOption, QStandardPathsStandardLocation,
};

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

pub fn find_kcfg_files(location: &QString) -> QStringList {
    let mut result = QStringList::default();

    let dirs = kcfg_dirs(location);

    for dir in Upcast::<QList<QString>>::upcast(&dirs) {
        let r = fs::read_dir(dir.to_string());
        if r.is_err() {
            println!("error: {:?}", r.err().unwrap());
            continue;
        }

        let listing = r.unwrap();

        for file in listing {
            let name: String = file.unwrap().path().to_str().unwrap().to_string();
            result.append(QString::from(&name));
        }
    }

    result.remove_duplicates();

    result
}
