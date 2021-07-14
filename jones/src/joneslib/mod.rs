pub mod display;
use prospector::objects::PythonClass;
use prospector::extract_python_class;
use prospector::grep_class;

use std::fs;
use std::path::PathBuf;

static CLASS_KEYWORD: &str = "class {template}";
static TEMPLATE_KEYWORD: &str = "{template}";
static PYTHON_EXTENSION: &str = "py";

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
    let full_class_name = CLASS_KEYWORD.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            return file_content.contains(&full_class_name)
        },
        Err(_) => {
            return false
        }
    };
}

/// Searches recurssively through a project for a Python class
pub fn project_traversal(dir_path: &PathBuf, class_name: &String) -> Option<PythonClass> {
    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None
        }
    };

    for file in current_dir {
        let file_path = file.unwrap().path();
        let file_path_name = file_path.to_str().unwrap();
        if file_path.is_dir() {
            match project_traversal(&file_path, class_name) {
                Some(value) => {
                   return Some(value)
                },
                None => continue
            };
        } else {
            match file_path.extension() {
                Some(extension) => {
                    if extension != PYTHON_EXTENSION {
                        continue
                    }
                },
                None => continue
            };
            if check_file_contains_class(class_name, &file_path_name){
                let file_content = match fs::read_to_string(file_path) {
                    Ok(content) => content,
                    Err(_) => {
                        println!("Now skipping");
                        continue
                    }
                };
                let lines: Vec<&str> = file_content.split("\n").collect();

                return Some(extract_python_class(lines, class_name))
            }
        }
    }
    return None
}

/// Project traversal recursive and searches for a keyword based on itself or on context (Phase 2)
pub fn smart_search(dir_path: &PathBuf, class_name: &String)  -> Option<Vec<ClassMatch>>{
    let mut found_matched_classes: Vec<ClassMatch> = Vec::new();

    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None
        }
    };

    for file in current_dir {
        let file_path = file.unwrap().path();
        let file_path_name = file_path.to_str().unwrap();
        if file_path.is_dir() {
            match smart_search(&file_path, class_name) {
                Some(matches) => found_matched_classes.extend(matches),
                None => continue
            };
        } else {
            match file_path.extension() {
                Some(extension) => {
                    if extension != PYTHON_EXTENSION {
                        continue
                    }
                },
                None => continue
            };
            let file_content = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(_) => continue
            };
            let lines: Vec<&str> = file_content.split("\n").collect();
            match grep_class(lines, &class_name, file_path_name) {
                Some(matches) => found_matched_classes.extend(matches),
                None => continue
            }
        }
    }

    if found_matched_classes.len() > 0 {
        Some(found_matched_classes)
    } else {
        None
    }
}
