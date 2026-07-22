// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cmake_package::find_package;
use cxx_qt_build::{CxxQtBuilder, QmlModule};

// list of (LibraryName, [LibraryTargets])
const LIBRARIES: &[(&str, &[&str])] = &[
    ("KF6Config", &["KF6::ConfigCore"]),
    ("KF6CoreAddons", &["KF6::CoreAddons"]),
];

fn main() {
    let mut builder = CxxQtBuilder::new_qml_module(
        QmlModule::new("org.kde.configeditor")
            .qml_file("qml/main.qml")
            .qml_file("qml/AppsPage.qml")
            .qml_file("qml/FilesPage.qml")
            .qml_file("qml/GroupsPage.qml")
            .qml_file("qml/EntriesPage.qml"),
    )
    .files([
        "src/appsmodel.rs",
        "src/filesmodel.rs",
        "src/groupsmodel.rs",
        "src/entrymodel.rs",
        "src/qstandardpaths.rs",
        "src/util.rs",
    ])
    .cpp_file("src/helper.cpp")
    .cpp_file("src/qstandardpaths.cpp");

    builder = link_libraries(builder);

    builder.build();
}

fn link_libraries(builder: CxxQtBuilder) -> CxxQtBuilder {
    let mut directories = Vec::new();

    for (name, targets) in LIBRARIES {
        match find_package(*name).find() {
            Err(err) => panic!("Cannot find {name}: {err:?}"),
            Ok(package) => {
                for target in *targets {
                    let cmake_target = package.target(target.to_owned()).unwrap();
                    cmake_target.link();
                    for dir in cmake_target.include_directories {
                        directories.push(dir);
                    }
                }
            }
        }
    }

    unsafe {
        builder.cc_builder(move |cc| {
            for dir in &directories {
                cc.include(dir);
            }
        })
    }
}
