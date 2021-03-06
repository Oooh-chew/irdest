// SPDX-FileCopyrightText: 2022 Katharina Fey <kookie@spacekookie.de>
// SPDX-FileCopyrightText: 2022 Lux <lux@lux.name>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

//! Ratman daemon entrypoint

#[macro_use]
extern crate tracing;

pub(crate) use ratman::{daemon::startup::*, *};

fn main() {
    let configuration = match daemon::config::Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(
                "Failed to load/write configuration: {}. Resuming with default values.",
                e
            );
            daemon::config::Config::new()
        }
    };

    let m = build_cli();
    let daemonize = m.is_present("DAEMONIZE");
    if daemonize {
        if let Err(err) = sysv_daemonize_app(m, configuration) {
            eprintln!("Ratmand suffered fatal error: {}", err);
            std::process::exit(-1);
        }
    } else if let Err(()) = async_std::task::block_on(run_app(m, configuration)) {
        std::process::exit(-1);
    }
}
