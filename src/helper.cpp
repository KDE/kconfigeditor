#include "helper.h"

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
