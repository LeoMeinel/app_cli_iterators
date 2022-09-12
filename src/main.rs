/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::{env, process};

use app_cli_iterators::{grep, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // Print to standard error stream
        eprintln!("ERROR: Problem parsing arguments! - {}", err);
        process::exit(1);
    });
    if let Err(err) = grep::grep(config) {
        eprintln!("ERROR: {}", err);
        process::exit(1);
    }
}
