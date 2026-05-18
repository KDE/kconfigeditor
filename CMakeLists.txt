# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: 2026 Nicolas Fella <nicolas.fella@gmx.de>

cmake_minimum_required(VERSION 3.29)
project(configeditor VERSION 0.1)

find_package(ECM 6.24.0 REQUIRED)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

include(KDEInstallDirs)
include(KDEClangFormat)
include(KDEGitCommitHooks)

find_package(Corrosion REQUIRED)

find_package(Qt6 REQUIRED COMPONENTS Core Quick QuickControls2)

find_package(KF6 REQUIRED COMPONENTS Kirigami Config)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_install(TARGETS configeditor ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})

install(FILES org.kde.configeditor.desktop DESTINATION ${KDE_INSTALL_APPDIR})
install(FILES org.kde.configeditor.svg DESTINATION ${KDE_INSTALL_FULL_ICONDIR}/hicolor/scalable/apps)
install(FILES org.kde.configeditor.metainfo.xml DESTINATION ${KDE_INSTALL_METAINFODIR})

file(GLOB_RECURSE ALL_CLANG_FORMAT_SOURCE_FILES *.cpp *.h)
kde_clang_format(${ALL_CLANG_FORMAT_SOURCE_FILES})

kde_configure_git_pre_commit_hook(CHECKS CLANG_FORMAT)
