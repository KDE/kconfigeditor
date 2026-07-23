// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{
    QColor, QDateTime, QFont, QList, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime,
    QUrl, QVariant, QVariantValue,
};
use entrymodel::EntryRoles;
use std::path::PathBuf;
use std::pin::Pin;

use std::fs;

use crate::config;
use crate::config::{Entry, Group, Kcfg, Label, Type};
use crate::util;

#[cxx_qt::bridge]
mod entrymodel {
    unsafe extern "C++" {
        include!(< QAbstractListModel >);
        type QAbstractListModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-qt-lib/qfont.h");
        type QFont = cxx_qt_lib::QFont;

        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;

        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;

        include!("cxx-qt-lib/qtime.h");
        type QTime = cxx_qt_lib::QTime;

        include!("cxx-qt-lib/qrect.h");
        type QRect = cxx_qt_lib::QRect;

        include!("cxx-qt-lib/qrectf.h");
        type QRectF = cxx_qt_lib::QRectF;

        include!("cxx-qt-lib/qsize.h");
        type QSize = cxx_qt_lib::QSize;

        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;

        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;

        include!("cxx-qt-lib/qlist.h");
        type QList_i32 = cxx_qt_lib::QList<i32>;

        include!("helper.h");
        #[rust_name = "entrytypes_to_variant"]
        fn typeToVariant(types: EntryTypes) -> QVariant;

        #[rust_name = "read_entry_string"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: &QString,
        ) -> QString;

        #[rust_name = "read_entry_int"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: i32) -> i32;

        #[rust_name = "read_entry_uint"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: u32) -> u32;

        #[rust_name = "read_entry_double"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: f64) -> f64;

        #[rust_name = "read_entry_bool"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: bool) -> bool;

        #[rust_name = "read_entry_font"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: QFont) -> QFont;

        #[rust_name = "read_entry_longlong"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: i64) -> i64;

        #[rust_name = "read_entry_rect"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: QRect) -> QRect;

        #[rust_name = "read_entry_rectf"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QRectF,
        ) -> QRectF;

        #[rust_name = "read_entry_size"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: QSize) -> QSize;

        #[rust_name = "read_entry_sizef"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QSizeF,
        ) -> QSizeF;

        #[rust_name = "read_entry_point"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QPoint,
        ) -> QPoint;

        #[rust_name = "read_entry_pointf"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QPointF,
        ) -> QPointF;

        #[rust_name = "read_entry_time"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: QTime) -> QTime;

        #[rust_name = "read_entry_url"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: QUrl) -> QUrl;

        #[rust_name = "read_entry_ulonglong"]
        fn readEntry(file: &QString, group: &QString, key: &QString, defaultValue: u64) -> u64;

        #[rust_name = "read_entry_int_list_as_variant"]
        fn readIntListEntryAsVariant(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: &QString,
        ) -> QVariant;

        #[rust_name = "read_entry_color"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QColor,
        ) -> QColor;

        #[rust_name = "read_entry_datetime"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QDateTime,
        ) -> QDateTime;

        #[rust_name = "read_entry_string_list"]
        fn readEntry(
            file: &QString,
            group: &QString,
            key: &QString,
            defaultValue: QStringList,
        ) -> QStringList;
    }

    #[qenum(EntryModel)]
    enum EntryRoles {
        Name,
        Type,
        Value,
        Choices,
        Label,
        Min,
        Max,
        DefaultValue,
    }

    #[qenum(EntryModel)]
    enum EntryTypes {
        String,
        Bool,
        Int,
        UInt,
        Enum,
        Font,
        IntList,
        StringList,
        DateTime,
        PathList,
        Double,
        Path,
        Color,
        Rect,
        LongLong,
        Size,
        Point,
        Url,
        Password,
        ULongLong,
        Time,
        RectF,
        PointF,
        SizeF,
        UrlList,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, group_name, cxx_name="groupName", READ, WRITE = set_group_name, NOTIFY = group_name_changed)]
        #[qproperty(QString, file_name, cxx_name="fileName", READ, WRITE = set_file_name, NOTIFY = file_name_changed)]
        #[qproperty(QString, location, READ, WRITE = set_location, NOTIFY = location_changed)]
        #[base = QAbstractListModel]
        type EntryModel = super::EntryModelRust;

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

        fn set_group_name(self: Pin<&mut Self>, group_name: QString);

        fn set_file_name(self: Pin<&mut Self>, file_name: QString);

        fn set_location(self: Pin<&mut Self>, location: QString);

        #[qsignal]
        #[cxx_name = "groupNameChanged"]
        fn group_name_changed(self: Pin<&mut Self>);

        #[qsignal]
        #[cxx_name = "fileNameChanged"]
        fn file_name_changed(self: Pin<&mut Self>);

        #[qsignal]
        #[cxx_name = "locationChanged"]
        fn location_changed(self: Pin<&mut Self>);
    }
}

pub struct EntryModelRust {
    entries: Vec<Entry>,
    group_name: QString,
    file_name: QString,
    location: QString,
}

impl Default for EntryModelRust {
    fn default() -> Self {
        Self {
            entries: vec![],
            group_name: QString::from(""),
            file_name: QString::from(""),
            location: QString::from(""),
        }
    }
}

use cxx_qt::CxxQtType;
use entrymodel::*;

impl entrymodel::EntryModel {
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.entries.len() as i32
    }

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = EntryRoles { repr: role };

        if let Some(entry) = self.entries.get(index.row() as usize) {
            match role {
                EntryRoles::Name => {
                    return QVariant::from(&QString::from(
                        &entry
                            .name
                            .clone()
                            .unwrap_or(entry.key.clone().unwrap_or_default()),
                    ));
                }
                EntryRoles::Type => {
                    return match entry.the_type {
                        Type::Int => entrytypes_to_variant(EntryTypes::Int),
                        Type::UInt => entrytypes_to_variant(EntryTypes::UInt),
                        Type::String => entrytypes_to_variant(EntryTypes::String),
                        Type::Bool => entrytypes_to_variant(EntryTypes::Bool),
                        Type::Font => entrytypes_to_variant(EntryTypes::Font),
                        Type::IntList => entrytypes_to_variant(EntryTypes::IntList),
                        Type::StringList => entrytypes_to_variant(EntryTypes::StringList),
                        Type::DateTime => entrytypes_to_variant(EntryTypes::DateTime),
                        Type::Enum => entrytypes_to_variant(EntryTypes::Enum),
                        Type::PathList => entrytypes_to_variant(EntryTypes::PathList),
                        Type::Double => entrytypes_to_variant(EntryTypes::Double),
                        Type::Path => entrytypes_to_variant(EntryTypes::Path),
                        Type::Color => entrytypes_to_variant(EntryTypes::Color),
                        Type::Rect => entrytypes_to_variant(EntryTypes::Rect),
                        Type::LongLong => entrytypes_to_variant(EntryTypes::LongLong),
                        Type::Size => entrytypes_to_variant(EntryTypes::Size),
                        Type::Point => entrytypes_to_variant(EntryTypes::Point),
                        Type::Url => entrytypes_to_variant(EntryTypes::Url),
                        Type::Password => entrytypes_to_variant(EntryTypes::Password),
                        Type::ULongLong => entrytypes_to_variant(EntryTypes::ULongLong),
                        Type::Time => entrytypes_to_variant(EntryTypes::Time),
                        Type::RectF => entrytypes_to_variant(EntryTypes::RectF),
                        Type::PointF => entrytypes_to_variant(EntryTypes::PointF),
                        Type::SizeF => entrytypes_to_variant(EntryTypes::SizeF),
                        Type::UrlList => entrytypes_to_variant(EntryTypes::UrlList),
                    };
                }
                EntryRoles::Value => {
                    let key = entry
                        .key
                        .clone()
                        .unwrap_or(entry.name.clone().unwrap_or_default());

                    let default_value = match &entry.default {
                        None => QVariant::default(),
                        Some(defs) => QVariant::from(&QString::from(&defs[0].0.clone())),
                    };

                    return match entry.the_type {
                        Type::Int => QVariant::from(&read_entry_int(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::UInt => QVariant::from(&read_entry_uint(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::String => QVariant::from(&read_entry_string(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            &default_value.value_or_default(),
                        )),
                        Type::Bool => QVariant::from(&read_entry_bool(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Font => QVariant::default(),
                        // Type::Font => QVariant::from(&read_entry_font(
                        //     &self.file_name,
                        //     &self.group_name,
                        //     &QString::from(key),
                        //     default_value.value_or_default(),
                        // )),
                        Type::IntList => read_entry_int_list_as_variant(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            &default_value.value_or_default(),
                        ),
                        Type::StringList => QVariant::from(&read_entry_string_list(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::DateTime => QVariant::from(&read_entry_datetime(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Enum => QVariant::default(),
                        Type::PathList => QVariant::default(),
                        Type::Path => QVariant::default(),
                        Type::Double => QVariant::from(&read_entry_double(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Color => QVariant::from(&read_entry_color(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Rect => QVariant::from(&read_entry_rect(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::LongLong => QVariant::from(&read_entry_longlong(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Size => QVariant::from(&read_entry_size(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Point => QVariant::from(&read_entry_point(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Url => QVariant::from(&read_entry_url(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Password => QVariant::from(&read_entry_string(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            &default_value.value_or_default(),
                        )),
                        Type::ULongLong => QVariant::from(&read_entry_ulonglong(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::RectF => QVariant::from(&read_entry_rectf(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::SizeF => QVariant::from(&read_entry_sizef(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::PointF => QVariant::from(&read_entry_pointf(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::Time => QVariant::from(&read_entry_time(
                            &self.file_name,
                            &self.group_name,
                            &QString::from(key),
                            default_value.value_or_default(),
                        )),
                        Type::UrlList => QVariant::default(),
                    };
                }
                EntryRoles::Label => {
                    return QVariant::from(&QString::from(
                        entry.label.clone().unwrap_or_default().label(),
                    ));
                }
                EntryRoles::DefaultValue => {
                    let key = entry
                        .key
                        .clone()
                        .unwrap_or(entry.name.clone().unwrap_or_default());

                    let default_value = match &entry.default {
                        None => QVariant::default(),
                        Some(defs) => QVariant::from(&QString::from(&defs[0].0.clone())),
                    };

                    return match entry.the_type {
                        Type::Int => Self::default_value::<i32>(entry),
                        Type::UInt => Self::default_value::<u32>(entry),
                        Type::String => Self::default_value::<QString>(entry),
                        Type::Bool => Self::default_value::<bool>(entry),
                        Type::Font => QVariant::default(), //Self::default_value::<QFont>(entry),
                        Type::IntList => QVariant::default(), // TODO
                        Type::StringList => Self::default_value::<QStringList>(entry),
                        Type::DateTime => Self::default_value::<QDateTime>(entry),
                        Type::Enum => Self::default_value::<QString>(entry),
                        Type::PathList => Self::default_value::<QStringList>(entry),
                        Type::Path => Self::default_value::<QString>(entry),
                        Type::Double => Self::default_value::<f64>(entry),
                        Type::Color => Self::default_value::<QColor>(entry),
                        Type::Rect => Self::default_value::<QRect>(entry),
                        Type::LongLong => Self::default_value::<i64>(entry),
                        Type::Size => Self::default_value::<QSize>(entry),
                        Type::Point => Self::default_value::<QPoint>(entry),
                        Type::Url => Self::default_value::<QUrl>(entry),
                        Type::Password => Self::default_value::<QString>(entry),
                        Type::ULongLong => Self::default_value::<u64>(entry),
                        Type::RectF => Self::default_value::<QRectF>(entry),
                        Type::SizeF => Self::default_value::<QSizeF>(entry),
                        Type::PointF => Self::default_value::<QPointF>(entry),
                        Type::Time => Self::default_value::<QTime>(entry),
                        Type::UrlList => QVariant::default(), //Self::default_value::<QList<QUrl>>(entry),
                    };
                }
                _ => {}
            }
        }
        QVariant::default()
    }

    fn default_value<T>(entry: &Entry) -> QVariant
    where
        T: QVariantValue,
    {
        let default_value = match &entry.default {
            None => QVariant::default(),
            Some(defs) => QVariant::from(&QString::from(&defs[0].0.clone())),
        };

        let value: T = default_value.value_or_default();

        return QVariant::from(&value);
    }

    fn role_names(&self) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(EntryRoles::Name.repr, "name".into());
        hash.insert(EntryRoles::Type.repr, "type".into());
        hash.insert(EntryRoles::Value.repr, "value".into());
        hash.insert(EntryRoles::Label.repr, "label".into());
        hash.insert(EntryRoles::DefaultValue.repr, "defaultValue".into());
        hash
    }

    fn rebuild(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();

        let kcfg_files = util::find_kcfg_files(&self.location);

        let configs: Vec<_> = Upcast::<QList<QString>>::upcast(&kcfg_files)
            .iter()
            .flat_map(|path| config::parse(&path.to_string().as_str()))
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

        self.as_mut().rust_mut().entries = configs
            .iter()
            .flat_map(|config| config.group.clone())
            .filter(|group| group.name == self.group_name.to_string())
            .flat_map(|group| group.entry.unwrap_or_default())
            .collect();

        // TODO merge duplicate entries

        self.end_reset_model();
    }

    pub fn set_group_name(mut self: Pin<&mut Self>, group_name: QString) {
        self.as_mut().rust_mut().group_name = group_name;
        self.as_mut().group_name_changed();

        self.rebuild();
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
