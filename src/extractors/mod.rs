pub mod utils;
pub mod objects;

static CLASS_KEYWORD: &str = "class {template}";
static TEMPLATE_KEYWORD: &str = "{template}";

/// Extracts the searched python class from the code
///
/// # Arguments
///
/// * `code_lines`: The full split into lines code file
///
/// * `class_name`: The searched class name
///
pub fn extract_python_class(code_lines: Vec<&str>, class_name: &str) -> objects::PythonClass {
    let full_class_name = CLASS_KEYWORD.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    let mut start_cutting: bool = false;
    let mut class_code_block: Vec<String> = Vec::new();

    for (counter, line) in code_lines.iter().enumerate() {
        if line.contains(&full_class_name) {
            start_cutting = true;
        }
        if start_cutting {
            class_code_block.push(line.to_string());
            if line.is_empty() && code_lines[counter-1].is_empty() {
                break
            }
        }
    }

    objects::PythonClass::new(class_code_block, class_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::objects::PythonClass;
    use super::extract_python_class;

    #[test]
    fn test_extract_python_class(){
        let python_file = "
class Human:

    def __init__(self, name: int):
        self.name == name

    def hi(self) -> None:
        print(f'My name is {self.name}')


class Kid:

    def __init__(self, age):
        self.age == age

    def hi(self):
        print(f'My name is {self.age}')
";
        let test_codebase = vec![
            "class Human:".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def hi(self):".to_string(),
            "        print(f'My name is {self.name}')".to_string(),
            "".to_string()
        ];
        let lines: Vec<&str> = python_file.split("\n").collect();

        let expected_class = PythonClass::new(test_codebase, String::from("Human"));

        assert_eq!(extract_python_class(lines, "Human"), expected_class);

    }
}