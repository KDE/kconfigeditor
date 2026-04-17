cmake_minimum_required(VERSION 3.29)
project(configeditor VERSION 0.1)

find_package(ECM 6.25.0 REQUIRED)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

include(KDEInstallDirs)

find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_install(TARGETS configeditor ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})
