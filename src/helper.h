#pragma once

#include <QVariant>

// #include "entrymodel.cxxqt.h"

template<typename T>
QVariant typeToVariant(T type)
{
    return QVariant::fromValue(type);
}

