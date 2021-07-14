use prospector::extract_method_name;
use prospector::extract_method_output;
use prospector::extract_parameters;
use prospector::extract_methods;
use prospector::extract_class_inheritance;
use prospector::objects::Method;
use prospector::objects::Parameter;

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

// TODO: Refactor and add negative test
#[test]
fn test_extract_method_output() {
    let test_string = String::from("def this_name(self, param2: int) -> List[int]:");
    let expected = String::from("List[int]");

    assert_eq!(extract_method_output(&test_string).unwrap(), expected);
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
fn test_extract_methods_multiple_lines(){
    let test_codebase = vec![
        "class Test:".to_string(),
        "".to_string(),
        "    def __init__(self, name: int,".to_string(),
        "                 param1: str,".to_string(),
        "                 param2: int) -> str:".to_string(),
        "        self.name = name".to_string(),
        "".to_string(),
        "    def say_hi(self):".to_string(),
        "        self.name = name".to_string(),
        "".to_string()
    ];

    let expected_methods = vec![
        Method::new(&"    def __init__(self, name: int, param1: str, param2: int) -> str:".to_string()),
        Method::new(&"    def say_hi(self):".to_string()),
    ];

    assert_eq!(extract_methods(test_codebase), expected_methods);
}

#[test]
fn test_extract_class_inheritance() {
    let test_header = String::from("class Human(Being, Earthling):");
    let expected = vec![String::from("Being"), String::from("Earthling")];

    assert_eq!(extract_class_inheritance(&test_header), Some(expected));
}

#[test]
fn test_extract_no_inheritance(){
    let test_header = String::from("class Human:");

    assert_eq!(extract_class_inheritance(&test_header), None);
}