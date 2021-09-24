use super::{DOCSTRING, NEWLINE};


pub fn extract_docstring(code_block: &Vec<String>) -> Option<String> {
    let docstring_vec = match get_docstring(&code_block) {
        Some(docstring) => docstring,
        None => return None
    };

    return Some(docstring_vec.join(NEWLINE))
}

fn get_docstring(code_block: &Vec<String>) -> Option<Vec<String>> {
    let mut start_docstring: bool = false;
    let mut docstring_vec: Vec<String> = Vec::new();

    for line in code_block.iter() {
        if !start_docstring && line.contains(DOCSTRING) {
            start_docstring = true;
            docstring_vec.push(format_line(line));
            match line.matches(DOCSTRING).count() {
                2 => break,
                _ => continue
            }
        }

        if start_docstring {
            docstring_vec.push(format_line(line));
            match line.contains(DOCSTRING) {
                true => break,
                false => continue
            }
        }
    }

    match docstring_vec.len() {
        0 => None,
        _ => Some(docstring_vec)
    }
}

fn format_line(line: &str) -> String {
    return line.trim().replace(DOCSTRING, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_line(){
        let test_string = "\"\"\"Test docstring";
        let expected = "Test docstring";

        assert_eq!(format_line(test_string), expected);
    }

    #[test]
    fn test_get_docstring(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    \"\"\"DocString\"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];
        let expected = vec!["DocString".to_string()];

        assert_eq!(get_docstring(&test_code_block), Some(expected));
    }

    #[test]
    fn test_get_docstring_none(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    pass".to_string(),
            "".to_string()
        ];
        assert_eq!(get_docstring(&test_code_block), None);
    }

    #[test]
    fn test_extract_docstring_some(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    \"\"\"".to_string(),
            "     DocString".to_string(),
            "     Some more test".to_string(),
            "    \"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];
        let expected = vec![
            "".to_string(),
            "DocString".to_string(),
            "Some more test".to_string(),
            "".to_string()
        ];

        assert_eq!(extract_docstring(&test_code_block), Some(expected.join(NEWLINE)));

    }
}