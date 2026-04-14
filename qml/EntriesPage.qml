// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import org.kde.kirigami as Kirigami

import org.kde.configeditor

Kirigami.ScrollablePage {
    id: root

    title: "Entries"

    required property string location
    required property string fileName
    required property string groupName

    ListView {

        anchors.fill: parent

        model: EntryModel {
            id: m

            location: root.location
            fileName: root.fileName
            groupName: root.groupName

        }

        delegate: DelegateChooser {
            id: chooser
            role: "type"

            DelegateChoice {
                roleValue: EntryModel.Bool
                ItemDelegate {
                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: model.name
                            subtitle: model.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: model.value !== model.defaultValue
                        }

                        Switch {
                            checked: model.value
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: model.value !== model.defaultValue
                        }
                    }
                }
            }

            DelegateChoice {
                roleValue: EntryModel.String

                ItemDelegate {
                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: model.name
                            subtitle: model.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: model.value !== model.defaultValue
                        }

                        TextField {
                            text: model.value
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: model.value !== model.defaultValue
                        }
                    }
                }
            }

            DelegateChoice {
                roleValue: EntryModel.Int

                ItemDelegate {
                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: model.name
                            subtitle: model.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: model.value !== model.defaultValue
                        }

                        SpinBox {
                            from: model.min ?? -2147483648
                            to: model.max ?? 2147483647
                            value: model.value
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: model.value !== model.defaultValue
                        }
                    }
                }
            }

            DelegateChoice {
                roleValue: EntryModel.Enum

                ItemDelegate {

                    id: del

                    property var choices: model.choices

                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: model.name
                            subtitle: model.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: model.value !== model.defaultValue
                        }

                        ComboBox {
                            model: del.choices
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: model.value !== model.defaultValue
                        }
                    }
                }
            }

            DelegateChoice {
                roleValue: EntryModel.StringList

                ItemDelegate {
                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: model.name
                            subtitle: model.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: model.value !== model.defaultValue
                        }

                        Text {
                            text: "This is supposed to be a StringList"
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: model.value !== model.defaultValue
                        }
                    }
                }
            }
        }
    }
}
