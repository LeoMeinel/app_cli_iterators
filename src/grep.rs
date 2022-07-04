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

use std::error::Error;
use std::fs;

use crate::Config;

pub fn grep(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterator methods instead of checking every element -> zero cost abstraction (same performance)
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterator methods instead of checking every element -> zero cost abstraction (same performance)
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Does it";
        let contents = "\
        What happens to a dream deferred?\n
        \nDoes it dry up\n\
        like a raisin in the sun?\n\
        Or fester like a sore—\n\
        And then run?\n\
        Does it stink like rotten meat?\n\
        Or crust and sugar over—\n\
        like a syrupy sweet?\n
        \nMaybe it just sags\n\
        like a heavy load.\n
        \nOr does it explode?\
        ";
        let expected_result = search_case_sensitive(query, contents);
        assert_eq!(
            &vec!["Does it dry up", "Does it stink like rotten meat?"],
            &expected_result
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "dOEs iT";
        let contents = "\
        What happens to a dream deferred?\n
        \nDoes it dry up\n\
        like a raisin in the sun?\n\
        Or fester like a sore—\n\
        And then run?\n\
        Does it stink like rotten meat?\n\
        Or crust and sugar over—\n\
        like a syrupy sweet?\n
        \nMaybe it just sags\n\
        like a heavy load.\n\
        \nOr does it explode?\
        ";
        let expected_result = search_case_insensitive(query, contents);
        assert_eq!(
            &vec![
                "Does it dry up",
                "Does it stink like rotten meat?",
                "Or does it explode?",
            ],
            &expected_result
        );
    }
}
