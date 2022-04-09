#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Katharina Fey <kookie@spacekookie.de>
#
# SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

set -xeo pipefail

# start ratmand with the state directory set
env XDG_DATA_HOME=state/multi-node/router2 ../target/debug/ratmand --inet '127.0.0.1:7000' -p 'inet#127.0.0.1:9000L' --no-discovery -v trace -b '127.0.0.1:7020' --no-webui
