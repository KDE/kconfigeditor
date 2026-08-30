// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

ItemDelegate {
    id: intDelegate

    required property string name
    required property string label
    required property int value
    required property int defaultValue

    width: ListView.view.width

    contentItem: RowLayout {
        Kirigami.TitleSubtitle {
            title: intDelegate.name
            subtitle: intDelegate.label
            Layout.fillWidth: true
        }

        Rectangle {
            color: "orange"
            implicitHeight: 10
            implicitWidth: 10
            radius: 5
            visible: spinBox.value !== intDelegate.defaultValue
        }

        SpinBox {
            id: spinBox

            from: intDelegate.min ?? -2147483648
            to: intDelegate.max ?? 2147483647
            value: intDelegate.value
        }

        Button {
            icon.name: "edit-undo"
            display: Button.IconOnly
            text: "Revert to default"
            enabled: intDelegate.value !== intDelegate.defaultValue
        }
    }
}
