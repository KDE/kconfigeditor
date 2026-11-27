// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>
// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL

#include "helper.h"

#include <QCoreApplication>
#include <QTimer>

#include <KLocalizedQmlContext>
#include <KLocalizedString>

using namespace std::literals;

// you might wonder what is going on here
// at the time of writing cxx-qt didn't support putting a QList<int> into a QVariant, so we have to do this detour through C++
// we receive the default value as string (because that's what we get from the kcfg file), but we need to interpret it as QList<int>. Then we read the entry and
// return it as a QVariant holding a QList<int>
QVariant readIntListEntryAsVariant(const QString &file, const QString &group, const QString &key, const QString &defaultValueString)
{
    KConfig c(file);
    QList<int> defaultValue = QVariant(defaultValueString).value<QList<int>>();
    return QVariant::fromValue<QList<int>>(c.group(group).readEntry(key, defaultValue));
}

void doSelfTest()
{
    QTimer::singleShot(std::chrono::milliseconds(500), qApp, &QCoreApplication::quit);
}

void setupI18n(QQmlApplicationEngine *engine)
{
    KLocalization::setupLocalizedContext(engine);
}
