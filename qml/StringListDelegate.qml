// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

BaseDelegate {
    id: stringListDelegate

    required property list<string> value
    required property list<string> defaultValue

    isDefault: stringListText.text !== stringListDelegate.defaultValue

    innerItem: TextField {
        id: stringListText

        text: stringListDelegate.value.join(";")
    }
}
