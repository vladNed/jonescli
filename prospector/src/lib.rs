pub mod objects;
mod utils;

use objects::Parameter;
use objects::Method;
use utils::regex_split;
pub use utils::grep_class;

static FUNCTION_KEYWORD: &str = " def ";
static DEFAULT_TYPE: &str = "None";
static ENDEF_KEYWORD: char = ':';
static CLASS_KEYWORD: &str = "class {template}";
static TEMPLATE_KEYWORD: &str = "{template}";

// Extract a python method name from its method header
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
pub fn extract_method_name(method_header: &String) -> Result<String, &str> {
    let split_header = regex_split(r"\W", true, method_header);
    if split_header[0].trim() != FUNCTION_KEYWORD.trim() {
        return Err("This is not a method header")
    }
    Ok(split_header[1].to_string())
}

/// Extract method output after the poiting arrow
///
/// # Arguments
///
/// * `header` - Python method header
///
/// # Output
///
/// * `Err` - if the header had no type and at split nothing happened
/// * `Ok` - returns header type
pub fn extract_method_output(header: &String) -> Result<String, &str> {
    let header_split = regex_split(r"( -> )", true, header);
    match header_split.len() {
        1 => {
            return Err("Output type not found")
        },
        _ => {
            let cleaned_output = header_split[1].trim().replace(":", "");
            return Ok(cleaned_output)
        }
    }
}

/// Extract method parameters with their static type
///
/// # Arguments
///
/// * `header`: - The header line from a Python method
pub fn extract_parameters(header: &String) -> Vec<Parameter> {

    // Split to get all the parameter
    let params_between_parantheses: Vec<&str> = regex_split(r"(\(|\):|\)\s)", true, header);
    let params: String = params_between_parantheses[1].to_string();
    let params_values: Vec<&str> = regex_split(r",", true, &params);
    let mut parameters: Vec<Parameter> = Vec::new();

    /*
    Starting from the second value, iterate and extract
    all the parameters from the method
    */
    for parameter in params_values.iter(){

        let parameter_string = parameter.to_string();
        if parameter.is_empty() {
            continue
        }

        let param_values: Vec<&str> = regex_split(r":\s", false, &parameter_string);
        match param_values.len() {
            1 => {
                parameters.push(
                    Parameter::new(
                        param_values[0].trim().to_string(),
                        DEFAULT_TYPE.to_string(),
                    )
                )
            },
            2 => {
                parameters.push(
                    Parameter::new(
                        param_values[0].trim().to_string(),
                        param_values[1].trim().to_string(),
                    )
                )
            },
            _ => println!("Found method in code with no parameters.")
        }

    }
    parameters
}

/// Extract methods found in a Python class
///
/// # Arguments
///
/// * `class_code` - The code for the Python class extracted from the
/// .py file
pub fn extract_methods(class_code: Vec<String>) -> Vec<Method> {

    // Initialize temp method and start for retrieving method headers
    // that span on multiple lines
    let mut methods: Vec<Method> = Vec::new();
    let mut temp_method = String::from("");
    let mut start = false;

    for line in class_code.iter() {
        if !start && line.contains(FUNCTION_KEYWORD) {
            start = true;
        }
        if start {
            temp_method.push_str(line.trim());
            let last_char = match temp_method.chars()
                .nth(temp_method.len() - 1) {
                    Some(chr) => chr,
                    None => continue
                };

            if last_char == ENDEF_KEYWORD {
                methods.push(Method::new(&temp_method));
                temp_method = String::from("");
                start = false;
            }
        }
    }
    methods
}

/// Extract class inheritance objects
///
/// > Note: Does not include extracting filters or constraints
///
/// # Arguments
///
/// * `line` - Is the class header previously extracted
///
/// # Output
///
/// A vector containing all the objects the class inherits
pub fn extract_class_inheritance(line: &String) -> Option<Vec<String>> {

    // Split the header for now to get objects that are inherited by the class
    let class_header_split: Vec<&str> = regex_split(r"(\(|\):|,)", true, line);
    if class_header_split.len() == 1 {
        return None
    }

    // Iterate through the split results but ommit the first entry
    let mut class_inheritance: Vec<String> = Vec::new();
    for inheritance in class_header_split[1..class_header_split.len()-1].iter(){
        let inherit_object = inheritance.trim().to_string();
        class_inheritance.push(inherit_object);
    }

    Some(class_inheritance)
}

// TODO: Make tests for this class
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
    let mut class_header: String = String::from("");

    for (counter, line) in code_lines.iter().enumerate() {
        if line.contains(&full_class_name) {
            start_cutting = true;
            class_header.push_str(line)
        }
        if start_cutting {
            class_code_block.push(line.to_string());
            if line.is_empty() && code_lines[counter-1].is_empty() {
                break
            }
        }
    }

    let class_inheritance = match extract_class_inheritance(&class_header) {
        Some(inheritance_vec) => inheritance_vec,
        None => Vec::new()
    };

    objects::PythonClass::new(class_code_block, class_name.to_string(), class_inheritance)
}