/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

pub mod display;
pub mod loader;
pub mod objects;
pub mod parser;

use std::fs;
use std::path::PathBuf;

const CLASS_TEMPLATE_INHERITANCE: &str = "class {template}(";
const CLASS_TEMPLATE: &str = "class {template}:";
const TEMPLATE_KEYWORD: &str = "{template}";
const PYTHON_EXTENSION: &str = "py";

type ClassMatch = (String, String);

/// Check if a file contains the searched class by reading the file.
///
/// # Arguments
/// * `class_name`: The class name of the Python class
/// * `file_path`: The file path to be read
///
/// # Errors
/// It panics if the file is cannot be read properly
fn check_file_contains_class(class_name: &str, file_path: &str) -> bool {
    let class_name_inheritance = CLASS_TEMPLATE_INHERITANCE
        .clone()
        .replace(TEMPLATE_KEYWORD, class_name);
    let class_name = CLASS_TEMPLATE.clone().replace(TEMPLATE_KEYWORD, class_name);

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            let first_check = file_content.contains(&class_name_inheritance);
            let second_check = file_content.contains(&class_name);
            return first_check || second_check;
        }
        Err(_) => return false,
    };
}

/// Searches recursively through a project for a Python class and extracts that
/// class into an PythonClass struct.
pub fn project_traversal(dir_path: &PathBuf, class_name: &String) -> Option<objects::PythonClass> {
    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None;
        }
    };

    for file in current_dir {
        let file_path = file.unwrap().path();
        let file_path_name = file_path.to_str().unwrap();
        if file_path.is_dir() {
            match project_traversal(&file_path, class_name) {
                Some(value) => return Some(value),
                None => continue,
            };
        } else {
            match file_path.extension() {
                Some(extension) => {
                    if extension != PYTHON_EXTENSION {
                        continue;
                    }
                }
                None => continue,
            }
            if check_file_contains_class(class_name, &file_path_name) {
                return loader::load_python_object(&file_path, &class_name);
            }
        }
    }
    return None;
}

/// Loads the project classes and filters them by the class name. Returns a vector
/// of tuples containing the class name and the file path.
pub fn search(path: &PathBuf, class_name: &String) -> Option<Vec<ClassMatch>> {
    let project_classes = match loader::load_python_project(path) {
        Some(classes) => classes,
        None => {
            println!("Error occurred while loading project classes");
            return None;
        }
    };

    let filtered_classes = project_classes
        .into_iter()
        .filter(|class| class.0.contains(class_name))
        .collect::<Vec<ClassMatch>>();

    Some(filtered_classes)
}

#[cfg(test)]
mod tests {
    use super::project_traversal;
    use std::fs;
    use std::path::PathBuf;

    static PYTHON_CODE: &str = "
    class God:
        \"\"\"DocString\"\"\"
        def __init__(self, name: int):
            self.name == name

        def hi(self) -> None:
            print(f'My name is {self.name}')

    ";
    static RANDOM_CODE: &str = "
    class TestClass:

        def __init__(self, age: gig):
            self.age == age

        def hi(self):
            print(f'My name is {self.age}')
    ";

    #[test]
    fn test_process_only_py_files() {
        // Paths
        let test_dir = String::from("./testing_none");
        let python_file = String::from("./testing_none/test.py");
        let random_file = String::from("./testing_none/test.rs");
        let mut pathbuf = PathBuf::new();

        // Create dir and files
        fs::create_dir(test_dir).expect("Could not write dir");
        fs::write(python_file, PYTHON_CODE).unwrap();
        fs::write(random_file, RANDOM_CODE).unwrap();
        pathbuf.push("./testing_none");

        // Assert
        assert_eq!(project_traversal(&pathbuf, &"TestCode".to_string()), None);

        // Destroy the test dir
        fs::remove_dir_all("./testing_none").expect("Could not delete dir");
    }
}
