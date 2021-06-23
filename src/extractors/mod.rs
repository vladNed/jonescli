use std::fs;

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
fn extract_python_class(code_lines: Vec<&str>, class_name: &str) -> objects::PythonClass {
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

/// Check if a file contains the searched class by reading the file.
///
/// # Arguments
/// * `class_name`: The class name of the Python class
/// * `file_path`: The file path to be read
///
/// # Errors
/// It panics if the file is cannot be read properly
fn check_file_contains_class(class_name: &str, file_path: &str) -> bool {
    let full_class_name = CLASS_KEYWORD.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            return file_content.contains(&full_class_name)
        },
        Err(err) => panic!("Could not read file: {}", err)
    };
}

/// Searches recurssively through a project for a Python class
pub fn project_traversal(dir_path: &String, class_name: &String) -> Option<objects::PythonClass> {
    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None
        }
    };

    for file_path in current_dir {
        let actual_file = file_path.unwrap();
        if actual_file.path().is_dir() {
            return project_traversal(&actual_file.path().to_str().unwrap().to_string(), class_name)
        } else {
            if check_file_contains_class(class_name, &actual_file.path().to_str().unwrap()) {
                let file_content = fs::read_to_string(actual_file.path()).expect("Could not read file");
                let lines: Vec<&str> = file_content.split("\n").collect();

                return Some(extract_python_class(lines, class_name))
            }
        }
    }
    return None
}

#[cfg(test)]
mod tests {
    use super::objects::PythonClass;
    use super::extract_python_class;
    use super::project_traversal;
    use super::check_file_contains_class;
    use std::fs;

    static PYTHON_CODE: &str = "
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

    #[test]
    fn test_extract_python_class(){

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
        let lines: Vec<&str> = PYTHON_CODE.split("\n").collect();

        let expected_class = PythonClass::new(test_codebase, String::from("Human"));

        assert_eq!(extract_python_class(lines, "Human"), expected_class);

    }

    #[test]
    fn test_check_file_contains_class() {
        let path = "./test.py";
        fs::write(path, PYTHON_CODE).unwrap();

        assert_eq!(true, check_file_contains_class("Human", path));
        fs::remove_file(path).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_file_contains_class_err() {
        check_file_contains_class("Human", "./iogh.py");
    }

    #[test]
    fn test_project_traversal() {
        let path = "./testing/test.py";
        fs::create_dir("./testing").expect("Could not write dire");
        fs::write(path, PYTHON_CODE).unwrap();


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

        let expected_class = PythonClass::new(test_codebase, String::from("Human"));
        assert_eq!(expected_class, project_traversal(&"./testing".to_string(), &"Human".to_string()).unwrap());

        fs::remove_dir_all("./testing").expect("Could not delete dir");
    }
}