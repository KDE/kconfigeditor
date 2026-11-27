#! /usr/bin/env bash

# SPDX-FileCopyrightText: None
# SPDX-License-Identifier: CC0-1.0

$XGETTEXT `find -name \*.cpp -o -name \*.rs -o -name \*.qml -o -name \*.js` -o $podir/kconfigeditor.pot
