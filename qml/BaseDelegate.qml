// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

ItemDelegate {
    id: root

    required property string name
    required property string label

    required property bool isDefault

    required property Item innerItem

    signal revertToDefault()

    width: ListView.view.width

    onInnerItemChanged: {
        innerItem.parent = inner
    }

    contentItem: RowLayout {
        Kirigami.TitleSubtitle {
            title: root.name
            subtitle: root.label
            Layout.fillWidth: true
        }

        Rectangle {
            color: "orange"
            implicitHeight: 10
            implicitWidth: 10
            radius: 5
            visible: !root.isDefault
        }

        Item {
            id: inner
            implicitWidth: childrenRect.width
            implicitHeight: childrenRect.height
            Layout.alignment: Qt.AlignVCenter
        }

        Button {
            icon.name: "edit-undo"
            display: Button.IconOnly
            text: "Revert to default"
            enabled: !root.isDefault
            onClicked: root.revertToDefault()
        }
    }
}
