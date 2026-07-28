// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#include "kconfig.h"
#include <KConfigGroup>

namespace rust::kf6::kconfig
{
std::unique_ptr<KConfig> from(const QString &file, KConfig::OpenFlags mode)
{
    return std::make_unique<KConfig>(file, mode);
}

KConfigGroup group(const KConfig &self, const QString &name)
{
    return self.group(name);
}
}
