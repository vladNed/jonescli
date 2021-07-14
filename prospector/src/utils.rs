use regex::Regex;

static CLASS_KEYWORD: &str = "class ";

/// Simple regex split on a given code line
/// # Arguments
///
/// * `r`: Regex string line
/// * `trim`: boolean if you want to trim the values
/// * `value`: value to split
pub fn regex_split<'a>(r: &'a str, trim: bool, value: &'a String) -> Vec<&'a str> {
    let regex_separator = Regex::new(r).expect("Invalid regex function given");
    if trim {
        regex_separator.split(value.trim()).collect()
    } else {
        regex_separator.split(value).collect()
    }
}

/// Search & find all Python classes that contain the keyword or are relevant to the context
///
/// # Arguments
///
/// * `lines` - The python file code lines previously read
/// * `keyword` - The class name given for the search
///
/// # Output
///
/// * `Option<Vec<String, String>>` - containing all the found relevant classes
pub fn grep_class<'a>(lines: Vec<&str>, keyword: &String, file_name: &str) -> Option<Vec<(String, String)>> {
    let mut found_match_classes: Vec<(String, String)> = Vec::new();
    for line in lines.iter() {
        if line.trim().starts_with(CLASS_KEYWORD) && line.contains(keyword) {
            found_match_classes.push(
                (line.to_string(), file_name.to_string())
            );
        }
    }
    if found_match_classes.len() > 0 {
        Some(found_match_classes)
    } else {
        None
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_regex_split_positive(){
        let test_string = String::from("test1:test2");
        let expected = vec!["test1", "test2"];
        let values: Vec<&str> = regex_split(r":", false, &test_string);

        assert_eq!(values, expected);
    }

    #[test]
    fn test_regex_split_trim(){
        let test_string = String::from("   test1:test2   ");
        let expected = vec!["test1", "test2"];
        let values: Vec<&str> = regex_split(r":", true, &test_string);

        assert_eq!(values, expected);
    }


    #[test]
    fn test_grep_class_some() {
        let test_codebase = vec![
            "class God:",
            "",
            "    def __init__(self, name):",
            "        self.name = name",
            "",
            "class GodMode:",
            "",
            "    def __init__(self, name):",
            "        self.name = name",
            "",
        ];

        let keyword = String::from("God");
        let filename = "./testing";
        let expected = vec![
            (String::from("class God:"), filename.to_string()),
            (String::from("class GodMode:"), filename.to_string()),
        ];

        assert_eq!(grep_class(test_codebase, &keyword, filename).unwrap(), expected);
    }

    #[test]
    fn test_grep_class_none() {
        let test_codebase = vec![
            "class God:",
            "",
            "    def __init__(self, name):",
            "        self.name = name",
            "",
            "class GodMode:",
            "",
            "    def __init__(self, name):",
            "        self.name = name",
            "",
        ];

        let keyword = String::from("Zeus");
        let filename = "./testing";


        assert_eq!(grep_class(test_codebase, &keyword, filename), None);
    }
}