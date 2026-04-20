// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{unsafe_impl_qflag, QFlags, QString, QStringList};

#[cxx_qt::bridge()]
mod ffi {

    #[derive(Debug)]
    #[repr(u32)]
    pub enum QStandardPathsStandardLocation {
        DesktopLocation,
        DocumentsLocation,
        FontsLocation,
        ApplicationsLocation,
        MusicLocation,
        MoviesLocation,
        PicturesLocation,
        TempLocation,
        HomeLocation,
        AppLocalDataLocation,
        CacheLocation,
        GenericDataLocation,
        RuntimeLocation,
        ConfigLocation,
        DownloadLocation,
        GenericCacheLocation,
        GenericConfigLocation,
        AppDataLocation,
        AppConfigLocation,
        PublicShareLocation,
        TemplatesLocation,
        StateLocation,
        GenericStateLocation,
    }

    #[derive(Debug)]
    #[repr(u32)]
    pub enum QStandardPathsLocateOption {
        LocateFile = 0x0,
        LocateDirectory = 0x1,
    }

    unsafe extern "C++" {
        include!("qstandardpaths.h");
        type QStandardPaths;
        type QStandardPathsStandardLocation;
        type QStandardPathsLocateOption;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        type QStandardPathsLocateOptions = super::QStandardPathsLocateOptions;

        #[rust_name = "qsp_set_test_mode_enabled"]
        fn qsp_setTestModeEnabled(testMode: bool);

        #[rust_name = "qsp_is_test_mode_enabled"]
        fn qsp_isTestModeEnabled() -> bool;

        #[rust_name = "qsp_writable_location"]
        fn qsp_writableLocation(location_type: QStandardPathsStandardLocation) -> QString;

        #[rust_name = "qsp_standard_locations"]
        fn qsp_standardLocations(location_type: QStandardPathsStandardLocation) -> QStringList;

        pub fn qsp_locate(
            location_type: QStandardPathsStandardLocation,
            file_name: &QString,
            options: QStandardPathsLocateOptions,
        ) -> QString;

        #[rust_name = "qsp_locate_all"]
        pub fn qsp_locateAll(
            location_type: QStandardPathsStandardLocation,
            file_name: &QString,
            options: QStandardPathsLocateOptions,
        ) -> QStringList;

        #[rust_name = "qsp_display_name"]
        pub fn qsp_displayName(location_type: QStandardPathsStandardLocation) -> QString;

        #[rust_name = "qsp_find_executable"]
        pub fn qsp_findExecutable(executable_name: &QString, paths: &QStringList) -> QString;
    }
}

pub use ffi::QStandardPaths;
pub use ffi::QStandardPathsLocateOption;
pub use ffi::QStandardPathsStandardLocation;

impl QStandardPaths {
    pub fn set_test_mode_enabled(test_mode: bool) {
        ffi::qsp_set_test_mode_enabled(test_mode);
    }

    pub fn is_test_mode_enabled() -> bool {
        ffi::qsp_is_test_mode_enabled()
    }

    pub fn writable_location(location_type: QStandardPathsStandardLocation) -> QString {
        ffi::qsp_writable_location(location_type)
    }

    pub fn standard_locations(location_type: QStandardPathsStandardLocation) -> QStringList {
        ffi::qsp_standard_locations(location_type)
    }

    pub fn locate(
        location_type: QStandardPathsStandardLocation,
        file_name: &QString,
        options: QStandardPathsLocateOptions,
    ) -> QString {
        ffi::qsp_locate(location_type, file_name, options)
    }

    pub fn locate_all(
        location_type: QStandardPathsStandardLocation,
        file_name: &QString,
        options: QStandardPathsLocateOptions,
    ) -> QStringList {
        ffi::qsp_locate_all(location_type, file_name, options)
    }

    pub fn display_name(location_type: QStandardPathsStandardLocation) -> QString {
        ffi::qsp_display_name(location_type)
    }

    pub fn find_executable(executable_name: &QString, paths: &QStringList) -> QString {
        ffi::qsp_find_executable(executable_name, paths)
    }
}

pub type QStandardPathsLocateOptions = QFlags<QStandardPathsLocateOption>;

unsafe_impl_qflag!(
    QStandardPathsLocateOption,
    "QStandardPathsLocateOptions",
    u32
);

#[cfg(test)]
mod test {
    use super::*;
    use cxx_qt_lib::CaseSensitivity;

    #[test]
    fn test() {
        QStandardPaths::set_test_mode_enabled(true);

        assert!(QStandardPaths::is_test_mode_enabled());

        assert!(
            QStandardPaths::writable_location(QStandardPathsStandardLocation::HomeLocation)
                .starts_with(&QString::from("/home"), CaseSensitivity::CaseSensitive)
        );

        assert!(
            QStandardPaths::find_executable(&QString::from("ls"), &QStringList::default())
                == QString::from("/usr/bin/ls")
        );
    }
}
