// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

import QtQuick
import QtQuick.Controls

BaseDelegate {
    id: intDelegate

    required property int value
    required property int defaultValue

    isDefault: spinBox.value === intDelegate.defaultValue

    innerItem: SpinBox {
        id: spinBox

        from: intDelegate.min ?? -2147483648
        to: intDelegate.max ?? 2147483647
        value: intDelegate.value
    }
}
