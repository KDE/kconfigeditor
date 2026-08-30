// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

ItemDelegate {
    id: boolDelegate

    required property string name
    required property string label
    required property bool value
    required property bool defaultValue

    width: ListView.view.width

    contentItem: RowLayout {
        Kirigami.TitleSubtitle {
            title: boolDelegate.name
            subtitle: boolDelegate.label
            Layout.fillWidth: true
        }

        Rectangle {
            color: "orange"
            implicitHeight: 10
            implicitWidth: 10
            radius: 5
            visible: boolSwitch.checked !== boolDelegate.defaultValue
        }

        Switch {
            id: boolSwitch

            checked: boolDelegate.value
        }

        Button {
            icon.name: "edit-undo"
            display: Button.IconOnly
            text: "Revert to default"
            enabled: boolSwitch.checked !== boolDelegate.defaultValue
        }
    }
}
