// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

ItemDelegate {
    id: stringListDelegate

    required property string name
    required property string label
    required property list<string> value
    required property list<string> defaultValue

    width: ListView.view.width

    contentItem: RowLayout {
        Kirigami.TitleSubtitle {
            title: stringListDelegate.name
            subtitle: stringListDelegate.label
            Layout.fillWidth: true
        }

        Rectangle {
            color: "orange"
            implicitHeight: 10
            implicitWidth: 10
            radius: 5
            visible: stringListText.text !== stringListDelegate.defaultValue
        }

        TextField {
            id: stringListText

            text: stringListDelegate.value.join(";")
        }

        Button {
            icon.name: "edit-undo"
            display: Button.IconOnly
            text: "Revert to default"
            enabled: stringListText.text !== stringListDelegate.defaultValue
        }
    }
}
