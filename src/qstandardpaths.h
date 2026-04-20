// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QStandardPaths>

using QStandardPathsStandardLocation = QStandardPaths::StandardLocation;
using QStandardPathsLocateOption = QStandardPaths::LocateOption;
using QStandardPathsLocateOptions = QStandardPaths::LocateOptions;

inline void qsp_setTestModeEnabled(bool testMode)
{
    QStandardPaths::setTestModeEnabled(testMode);
}

inline bool qsp_isTestModeEnabled()
{
    return QStandardPaths::isTestModeEnabled();
}

inline QString qsp_writableLocation(QStandardPaths::StandardLocation type)
{
    return QStandardPaths::writableLocation(type);
}

inline QStringList qsp_standardLocations(QStandardPaths::StandardLocation type)
{
    return QStandardPaths::standardLocations(type);
}

inline QString qsp_locate(QStandardPaths::StandardLocation type, const QString &fileName, QStandardPaths::LocateOptions options)
{
    return QStandardPaths::locate(type, fileName, options);
}

inline QStringList qsp_locateAll(QStandardPaths::StandardLocation type, const QString &fileName, QStandardPaths::LocateOptions options)
{
    return QStandardPaths::locateAll(type, fileName, options);
}

inline QString qsp_displayName(QStandardPaths::StandardLocation type)
{
    return QStandardPaths::displayName(type);
}

inline QString qsp_findExecutable(const QString &executableName, const QStringList &paths)
{
    return QStandardPaths::findExecutable(executableName, paths);
}
