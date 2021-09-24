/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

use regex::Regex;

const OPEN_PARANTHESES: char = '[';
const CLOSE_PARANTHESES: char = ']';
const COMMA: char = ',';


/// Fetch the arguments from a class method and mark the commas
/// that will be used for splitting further
///
/// # Arguments
///
/// * `header` - The method header
///
pub fn get_header_arguments(header: &String) -> Option<String> {
    let reg = Regex::new(r".*?\(|\).*").unwrap();
    let segments: Vec<&str> = reg.split(header)
        .filter(|&entry| !entry.is_empty())
        .collect();

    match segments.len() {
        0 => None,
        _ => Some(mark_commas_for_split(segments[0]))
    }
}

/// Mark the commas that are not dividing the arguments in a method
/// class
///
/// # Arguments
///
/// * `args` - Arguments segment from a method header
fn mark_commas_for_split(args: &str) -> String {
    let mut inside_brackets: bool = false;
    let mut marked_args = args.to_string();

    for (pos, ch) in args.chars().enumerate() {
        match ch {
            OPEN_PARANTHESES => inside_brackets = true,
            CLOSE_PARANTHESES => inside_brackets = false,
            COMMA => {
                if inside_brackets {
                    marked_args.remove(pos+1);
                }
            },
            _ => continue
        }
    }

    marked_args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_commas_for_split() {
        let args = "param1: str, param2: Dict[str, int]";
        let expected = String::from("param1: str, param2: Dict[str,int]");

        assert_eq!(mark_commas_for_split(args), expected);
    }

    #[test]
    fn test_get_header_arguments(){
        let header = String::from("def test_method(param1: str, param2: Dict[str, int]) -> str");
        let expected = String::from("param1: str, param2: Dict[str,int]");

        assert_eq!(get_header_arguments(&header), Some(expected));
    }

    #[test]
    fn test_get_header_no_arguments(){
        let header = String::from("def test_method() -> str");
        assert_eq!(get_header_arguments(&header), None);
    }
}