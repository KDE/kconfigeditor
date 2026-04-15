// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use libflatpak::gio::Cancellable;
use libflatpak::glib::GString;
use libflatpak::prelude::{InstallationExt, InstalledRefExt, RefExt};
use libflatpak::{Installation, RefKind};

use appsmodel::AppsRoles;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
mod appsmodel {
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

    #[qenum(AppsModel)]
    enum AppsRoles {
        Id,
        Name,
        Location,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        type AppsModel = super::AppsModelRust;

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
    }
}

struct App {
    id: String,
    name: String,
    location: String,
}

pub struct AppsModelRust {
    apps: Vec<App>,
}

fn g_to_q(input: &GString) -> QString {
    QString::from(input.as_str())
}

impl Default for AppsModelRust {
    fn default() -> Self {
        let mut apps = vec![];

        apps.push(App {
            id: String::from("global"),
                  name: String::from("Global"),
                  location: String::default(),
        });

        let cancellable = Cancellable::new();

        let installs = libflatpak::system_installations(Some(&cancellable)).unwrap();

        for install in &installs {
            let refs = install
                .list_installed_refs_by_kind(RefKind::App, Some(&cancellable))
                .unwrap();

            for r in refs {
                apps.push(App {
                    id: r.name().unwrap().into(),
                    name: r.appdata_name().unwrap().into(),
                    location: r.deploy_dir().unwrap().into(),
                });
            }
        }

        let user_install = Installation::new_user(Some(&cancellable)).unwrap();

        let refs = user_install
            .list_installed_refs_by_kind(RefKind::App, Some(&cancellable))
            .unwrap();

        for r in refs {
            apps.push(App {
                id: r.name().unwrap().into(),
                name: r.appdata_name().unwrap_or(r.name().unwrap()).into(),
                location: r.deploy_dir().unwrap().into(),
            });
        }

        Self { apps }
    }
}

use appsmodel::*;
use cxx_qt::CxxQtType;

impl appsmodel::AppsModel {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.apps.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = AppsRoles { repr: role };

        if let Some(app) = self.apps.get(index.row() as usize) {
            match role {
                AppsRoles::Id => {
                    return QVariant::from(&QString::from(&app.id));
                }
                AppsRoles::Name => {
                    return QVariant::from(&QString::from(&app.name));
                }
                AppsRoles::Location => {
                    return QVariant::from(&QString::from(&app.location));
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(AppsRoles::Name.repr, "name".into());
        hash.insert(AppsRoles::Location.repr, "location".into());
        hash
    }
}
