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
            }

            DelegateChoice {
                roleValue: EntryModel.String

                ItemDelegate {
                    id: stringDelegate

                    required property string name
                    required property string label
                    required property string value
                    required property string defaultValue

                    width: ListView.view.width

                    contentItem: RowLayout {
                        Kirigami.TitleSubtitle {
                            title: stringDelegate.name
                            subtitle: stringDelegate.label
                            Layout.fillWidth: true
                        }

                        Rectangle {
                            color: "orange"
                            implicitHeight: 10
                            implicitWidth: 10
                            radius: 5
                            visible: stringTextField.text !== stringDelegate.defaultValue
                        }

                        TextField {
                            id: stringTextField

                            text: stringDelegate.value
                        }

                        Button {
                            icon.name: "edit-undo"
                            display: Button.IconOnly
                            text: "Revert to default"
                            enabled: stringTextField.text !== stringDelegate.defaultValue
                        }
                    }
                }
            }

            DelegateChoice {
                roleValue: EntryModel.Int

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
            }

            DelegateChoice {
                roleValue: EntryModel.Enum

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
            }

            DelegateChoice {
                roleValue: EntryModel.StringList

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
            }
        }
    }
}
