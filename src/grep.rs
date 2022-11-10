/*
 * File: grep.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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

#[allow(clippy::needless_borrow)]
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
