// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

Kirigami.ScrollablePage {
    id: groupsPage

    required property string fileName
    required property string location

    title: "Groups"

    ListView {
        anchors.fill: parent

        model: GroupsModel {
            location: groupsPage.location
            fileName: groupsPage.fileName
        }

        delegate: ItemDelegate {
            required property string name

            width: ListView.view.width
            text: name

            onClicked: pageStack.push(Qt.resolvedUrl("EntriesPage.qml"), {
                location: location,
                fileName: fileName,
                groupName: name
            })
        }
    }
}
