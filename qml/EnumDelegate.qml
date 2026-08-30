// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

BaseDelegate {
    id: enumDelegate

    required property string value
    required property string defaultValue
    required property list<string> choices

    isDefault: enumCombo.currentValue === enumDelegate.defaultValue

    innerItem: ComboBox {
        id: enumCombo

        model: enumDelegate.choices
    }
}
