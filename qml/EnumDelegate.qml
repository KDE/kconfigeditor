// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

ItemDelegate {
    id: enumDelegate

    required property string name
    required property string label
    required property string value
    required property string defaultValue
    required property list<string> choices

    width: ListView.view.width

    contentItem: RowLayout {
        Kirigami.TitleSubtitle {
            title: enumDelegate.name
            subtitle: enumDelegate.label
            Layout.fillWidth: true
        }

        Rectangle {
            color: "orange"
            implicitHeight: 10
            implicitWidth: 10
            radius: 5
            visible: enumCombo.currentValue !== enumDelegate.defaultValue
        }

        ComboBox {
            id: enumCombo

            model: enumDelegate.choices
        }

        Button {
            icon.name: "edit-undo"
            display: Button.IconOnly
            text: "Revert to default"
            enabled: enumCombo.currentValue !== enumDelegate.defaultValue
        }
    }
}
