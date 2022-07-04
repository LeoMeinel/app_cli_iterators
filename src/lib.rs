/*
 * app_cli_iterators is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/app_cli_iterators/blob/main/LICENSE
 */

use std::env;

pub mod grep;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Take iterator instead of reference to String -> eliminates use of .clone()
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't find query string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Can't find file name."),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
