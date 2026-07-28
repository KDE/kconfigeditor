// SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fell@gmx.de>
// SPDX-License-Identifier: MPL-2.0

#pragma once

#include <KConfig>

namespace rust::kf6::kconfig
{
std::unique_ptr<KConfig> from(const QString &file, KConfig::OpenFlags mode);

KConfigGroup group(const KConfig &self, const QString &name);

using KConfigAccessMode = KConfigBase::AccessMode;
using OpenFlag = KConfig::OpenFlag;
using OpenFlags = KConfig::OpenFlags;
}
