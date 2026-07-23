// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

mod appsmodel;
mod config;
mod entrymodel;
mod filesmodel;
mod groupsmodel;
mod qstandardpaths;
mod util;

use std::fs;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QStringList, QUrl};
use cxx_qt_lib_extras::{QCommandLineOption, QCommandLineParser};

fn main() {
    let mut parser = QCommandLineParser::default();
    let check_files_option = QCommandLineOption::from(&QString::from("check-files"));
    parser.add_option(&check_files_option);

    // TODO put QGuiApplication::arguments() in cxx-qt
    let mut args = QStringList::default();
    for arg in std::env::args_os() {
        args.append(QString::from(arg.into_string().unwrap()));
    }

    let mut app = QGuiApplication::new();

    parser.process(&args);

    if parser.is_set(&QString::from("check-files")) {
        let kcfg_files = util::find_kcfg_files(&QString::default());

        let config_schemas: Vec<_> = kcfg_files
            .iter()
            .flat_map(|file| config::parse(&file.to_string()))
            .collect();

        let file_names_from_schemas: Vec<_> = config_schemas
            .iter()
            .flat_map(|config| config.kcfgfile.clone())
            .flat_map(|cfg| cfg.name)
            .collect();

        let config_files = fs::read_dir(std::env::var("HOME").unwrap() + "/.config/").unwrap();

        println!("The following files have no associated kcfg file:");

        for config_file in config_files {
            if config_file.as_ref().unwrap().file_type().unwrap().is_dir() {
                continue;
            }

            if !file_names_from_schemas.contains(
                &config_file
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap(),
            ) {
                println!(
                    "{}",
                    &config_file.unwrap().file_name().into_string().unwrap()
                );
            }
        }
    } else {
        let mut engine = QQmlApplicationEngine::new();

        if let Some(engine) = engine.as_mut() {
            engine.load(&QUrl::from("qrc:/qt/qml/org/kde/configeditor/qml/main.qml"));
        }

        if let Some(app) = app.as_mut() {
            app.exec();
        }
    }
}
