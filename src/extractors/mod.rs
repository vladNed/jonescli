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