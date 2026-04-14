// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cxx_qt_build::{CxxQtBuilder, QmlModule};
fn main() {
    CxxQtBuilder::new_qml_module(
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
    ])
    .cpp_file("src/helper.cpp")
    .build();
}
