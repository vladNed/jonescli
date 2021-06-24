use regex::Regex;
use super::objects::{Parameter, Method};

static FUNCTION_KEYWORD: &str = " def ";
static DEFAULT_TYPE: &str = "None";

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
pub fn extract_method_name(method_header: &String) -> Result<String, &str> {
    let split_header = regex_split(r"\W", true, method_header);
    if split_header[0].trim() != FUNCTION_KEYWORD.trim() {
        return Err("This is not a method header")
    }
    Ok(split_header[1].to_string())
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
    let mut methods: Vec<Method> = Vec::new();
    for line in class_code.iter() {
        if line.contains(FUNCTION_KEYWORD) {
            methods.push(Method::new(line))
        }
    }
    methods
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



#[cfg(test)]
mod tests {
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
    fn test_extract_method_name(){
        let test_string = String::from("def this_name(self, param2: int) -> None:");
        let expected = String::from("this_name");

        assert_eq!(extract_method_name(&test_string).unwrap(), expected);
    }

    #[test]
    fn test_extract_method_name_negative(){
        let test_string = String::from("import definition as positive");
        assert!(extract_method_name(&test_string).is_err());
    }

    #[test]
    fn test_extract_parameters_positive(){
        let test_string = String::from("def this_name(param1: str, param2: int) -> None:");
        let expected_parameters = vec![
            Parameter::new(String::from("param1"), String::from("str")),
            Parameter::new(String::from("param2"), String::from("int")),
        ];

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_parameters_one_parameter(){
        let test_string = String::from("def this_name(self) -> None:");
        let expected_parameters = vec![
            Parameter::new(String::from("self"), String::from("None")),
        ];

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_parameters_no_parameter(){
        let test_string = String::from("def this_name() -> None:");
        let expected_parameters = Vec::new();

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_methods_positive(){
        let test_codebase = vec![
            "class Test:".to_string(),
            "".to_string(),
            "    def __init__(self, name):".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def say_hi(self):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];

        let expected_methods = vec![
            Method::new(&"    def __init__(self, name):".to_string()),
            Method::new(&"    def say_hi(self):".to_string()),
        ];

        assert_eq!(extract_methods(test_codebase), expected_methods);
    }

    #[test]
    fn test_extract_method_output() {
        let test_string = String::from("def this_name(self, param2: int) -> List[int]:");
        let expected = String::from("List[int]");

        assert_eq!(extract_method_output(&test_string).unwrap(), expected);
    }
}