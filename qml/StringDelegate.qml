// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

BaseDelegate {
    id: stringDelegate

    required property string value
    required property string defaultValue

    isDefault: stringTextField.text === stringDelegate.defaultValue

    innerItem: TextField {
        id: stringTextField

        text: stringDelegate.value
    }
}
