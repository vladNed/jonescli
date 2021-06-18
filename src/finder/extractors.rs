use regex::Regex;
use super::objects::{Parameter, Method};

static CLASS_KEYWORD: &str = "class {template}";
static TEMPLATE_KEYWORD: &str = "{template}";
static SELF_KEYWORD: &str = "self";
static FUNCTION_KEYWORD: &str = " def ";


/// Extracts the searched python class from the code
///
/// # Arguments
///
/// * `code_lines`: The full split into lines code file
///
/// * `class_name`: The searched class name
///
/// # Output
///
/// ```rust
/// let class_code_block: Vec<String>
/// ```
pub fn extract_python_class(code_lines: Vec<&str>, class_name: &str) -> Vec<String> {
    let full_class_name = CLASS_KEYWORD.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    let mut start_cutting: bool = true;
    let mut class_code_block: Vec<String> = Vec::new();

    for (counter, line) in code_lines.iter().enumerate() {
        if line.contains(&full_class_name) { start_cutting = true; }
        if start_cutting { class_code_block.push(line.to_string()) }
        if start_cutting && (line.is_empty() && code_lines[counter-1].is_empty()) {
            break;
        }
    }

    class_code_block
}

/// Simple regex split on a given code line
/// # Arguments
///
/// * `r`: Regex string line
/// * `trim`: boolean if you want to trim the values
/// * `value`: value to split
fn regex_split<'a>(r: &'a str, trim: bool, value: &'a String) -> Vec<&'a str> {
    let regex_separator = Regex::new(r).expect("Invalid regex function given");
    if trim {
        regex_separator.split(value.trim()).collect()
    } else {
        regex_separator.split(value).collect()
    }
}

/// Extract a python method name from its method header
///
/// # Arguments
///
/// * `method_header`: Method header code line
///
/// # Example
/// ```python
/// def method_name(self, arg1: int, arg2: str) -> None:
/// ```
/// Extracted name here is `method_name`
///
/// # Output
///
/// ```rust
/// let header_name: String
/// ```
pub fn extract_method_name(method_header: &String) -> String {
    let split_header = regex_split(r"\W", true, method_header);

    split_header[1].to_string()
}

pub fn extract_parameters(header: &String) -> Vec<Parameter> {
    // Split to get all the parameter
    let params_values: Vec<&str> = regex_split(r"(\(|\)|,)", true, header);
    let mut parameters: Vec<Parameter> = Vec::new();

    /*
    Starting from the second value, iterate and extract
    all the parameters from the method
    */
    for parameter in params_values[1..].iter(){
        if parameter.trim().contains("self") {
            parameters.push(
                Parameter::new(
                    SELF_KEYWORD.to_string(),
                    String::new(),
                )
            );
            continue
        }
        let parameter_string = parameter.to_string();
        let param_values: Vec<&str> = regex_split(r"\:\s", false, &parameter_string);
        parameters.push(
            Parameter::new(
                param_values[0].to_string(),
                param_values[1].to_string(),
            )
        )

    }
    parameters
}

pub fn extract_methods(class: Vec<String>) -> Vec<Method> {
    let mut functions: Vec<Method> = Vec::new();
    for line in class.iter() {
        if line.contains(FUNCTION_KEYWORD) {
            functions.push(Method::new(line))
        }
    }
    functions
}