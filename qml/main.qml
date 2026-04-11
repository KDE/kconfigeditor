// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

Kirigami.ApplicationWindow {
    visible: true

    pageStack.initialPage: appsList

    Component {
        id: appsList

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

                    onClicked: pageStack.push(filesList, {
                        appId: name,
                        location: location,
                    })
                }
            }
        }
    }

    Component {
        id: filesList

        Kirigami.ScrollablePage {
            id: filesPage

            required property string appId
            required property string location

            title: "Files"

            ListView {
                anchors.fill: parent

                model: FilesModel {
                    location: filesPage.location
                }

                delegate: ItemDelegate {
                    required property string name

                    width: ListView.view.width
                    text: name
                }
            }
        }
    }
}
