// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

mod appsmodel;
mod config;
mod filesmodel;
mod groupsmodel;
mod entrymodel;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/org/kde/configeditor/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
