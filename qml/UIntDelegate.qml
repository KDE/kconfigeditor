// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

BaseDelegate {
    id: uintDelegate

    required property int value
    required property int defaultValue

    required property int min
    required property int max

    isDefault: spinBox.value === uintDelegate.defaultValue

    innerItem: SpinBox {
        id: spinBox

        from: uintDelegate.min
        to: uintDelegate.max
        value: uintDelegate.value
    }
}
