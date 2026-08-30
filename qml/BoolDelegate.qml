// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

import org.kde.configeditor

BaseDelegate {
    id: boolDelegate

    required property bool value
    required property bool defaultValue

    isDefault: boolSwitch.checked === boolDelegate.defaultValue

    innerItem: Switch {
        id: boolSwitch

        horizontalPadding: 0
        checked: boolDelegate.value
    }
}
