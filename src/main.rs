// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

mod appsmodel;
mod config;
mod entrymodel;
mod filesmodel;
mod groupsmodel;
mod kconfig;
mod kconfiggroup;
mod qstandardpaths;
mod util;

use std::fs;

use cxx_qt_lib::{
    CaseSensitivity, QGuiApplication, QQmlApplicationEngine, QString, QStringList, QUrl,
};
use cxx_qt_lib_extras::{QCommandLineOption, QCommandLineParser};

use crate::config::Group;
use crate::config::Kcfg;
use crate::kconfig::KConfig;
use crate::kconfig::OpenFlags;
use crate::kconfiggroup::KConfigGroup;

fn groups_for_file(schemas: &Vec<Kcfg>, file_name: &str) -> Vec<Group> {
    schemas
        .iter()
        .filter(|&config| config.kcfgfile.is_some())
        .filter(|&config| {
            config
                .kcfgfile
                .as_ref()
                .unwrap()
                .name
                .clone()
                .unwrap_or_default()
                == file_name
        })
        .map(|config| config.group.clone())
        .flatten()
        .collect()
}

fn file_names_from_schemas(schemas: &Vec<Kcfg>) -> Vec<String> {
    schemas
        .iter()
        .flat_map(|config| config.kcfgfile.clone())
        .flat_map(|cfg| cfg.name)
        .collect()
}

fn print_missing_files(schemas: &Vec<Kcfg>) {
    let config_files = fs::read_dir(std::env::var("HOME").unwrap() + "/.config/").unwrap();

    for config_file in config_files {
        if config_file.as_ref().unwrap().file_type().unwrap().is_dir() {
            continue;
        }

        if !file_names_from_schemas(schemas).contains(
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
}

fn missing_keys_in_group(schemas: &Vec<Kcfg>, group: &KConfigGroup) -> Vec<String> {
    let keys = group.key_list();

    let group_schemas = unsafe {
        groups_for_file(
            schemas,
            &group.config().as_ref_unchecked().name().to_string(),
        )
    };

    let group_contains_entry = |group: &Group, key: &QString| {
        if group.entry.is_none() {
            return false;
        }
        return group
            .entry
            .clone()
            .unwrap()
            .iter()
            .find(|entry| {
                &key.to_string() == &entry.name.clone().unwrap_or_default()
                    || &key.to_string() == &entry.key.clone().unwrap_or_default()
            })
            .is_some();
    };

    keys.iter()
        .filter(|key| {
            group_schemas
                .iter()
                .find(|group| group_contains_entry(group, key))
                .is_none()
        })
        .map(|qs| qs.to_string())
        .collect()
}

fn print_missing_keys(schemas: &Vec<Kcfg>) {
    let config_files = fs::read_dir(std::env::var("HOME").unwrap() + "/.config/").unwrap();

    for config_file in config_files {
        if config_file.as_ref().unwrap().file_type().unwrap().is_dir() {
            continue;
        }

        let file_name = config_file.unwrap().file_name().into_string().unwrap();

        if file_names_from_schemas(schemas).contains(&file_name) {
            println!("{:?}:", file_name);

            let config = KConfig::from(&QString::from(file_name), OpenFlags::NoGlobals);

            for group_name in config.group_list().iter() {
                let group = config.group(&group_name);
                let missing = missing_keys_in_group(schemas, &group);

                for key in missing {
                    println!("{}: {}", group_name, key);
                }
            }

            println!();
        }
    }
}

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

        println!("The following files have no associated kcfg file:");

        print_missing_files(&config_schemas);

        println!();

        println!("The following keys are unaccounted for:");

        print_missing_keys(&config_schemas);
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
