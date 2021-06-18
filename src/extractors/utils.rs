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
    let params_values: Vec<&str> = regex_split(r"(\(|\):|,)", true, header);
    let mut parameters: Vec<Parameter> = Vec::new();


    /*
    Starting from the second value, iterate and extract
    all the parameters from the method
    */
    for parameter in params_values[1..params_values.len()-1].iter(){

        let parameter_string = parameter.to_string();
        let param_values: Vec<&str> = regex_split(r":\s", false, &parameter_string);

        match param_values.len() {
            1 => {
                parameters.push(
                    Parameter::new(
                        param_values[0].to_string(),
                        DEFAULT_TYPE.to_string(),
                    )
                )
            },
            2 => {
                parameters.push(
                    Parameter::new(
                        param_values[0].to_string(),
                        param_values[1].to_string(),
                    )
                )
            },
            _ => println!("Found method in code with no parameters.")
        }

    }
    parameters
}

pub fn extract_methods(class: Vec<String>) -> Vec<Method> {
    let mut methods: Vec<Method> = Vec::new();
    for line in class.iter() {
        if line.contains(FUNCTION_KEYWORD) {
            methods.push(Method::new(line))
        }
    }
    methods
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
}