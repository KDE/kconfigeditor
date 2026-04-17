#pragma once

#include <QVariant>

#include <KConfig>
#include <KConfigGroup>
#include <QString>

template<typename T>
QVariant typeToVariant(T type)
{
    return QVariant::fromValue(type);
}

template<typename T>
T readEntry(const QString &file, const QString &group, const QString &key, T defaultValue)
{
    KConfig c(file);
    return c.group(group).readEntry<T>(key, defaultValue);
}

template<typename T>
T readEntry(const QString &file, const QString &group, const QString &key, const T &defaultValue)
{
    KConfig c(file);
    return c.group(group).readEntry<T>(key, defaultValue);
}

QVariant readIntListEntryAsVariant(const QString &file, const QString &group, const QString &key, const QString &defaultValueString);
