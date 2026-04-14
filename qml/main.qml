// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

Kirigami.ApplicationWindow {
    visible: true

    pageStack.initialPage: AppsPage {}
}
