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
                BoolDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.String
                StringDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Color

                ColorDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Font

                FontDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Int

                IntDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.UInt

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Enum

                EnumDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.StringList

                StringListDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Url

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.IntList

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.DateTime

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.PathList

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Path

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Time

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Color

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Rect

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Double

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.LongLong

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Size

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Point

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.Password

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.ULongLong

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.RectF

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.SizeF

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.PointF

                TodoDelegate {}
            }

            DelegateChoice {
                roleValue: EntryModel.UrlList

                TodoDelegate {}
            }
        }
    }
}
