// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

Kirigami.ScrollablePage {

    title: "Apps"

    ListView {
        anchors.fill: parent

        model: AppsModel {}

        delegate: ItemDelegate {
            required property string name
            required property string location

            width: ListView.view.width
            text: name

            onClicked: pageStack.push(Qt.resolvedUrl("FilesPage.qml"), {
                appId: name,
                location: location
            })
        }
    }
}
