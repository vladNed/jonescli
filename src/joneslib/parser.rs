/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
use super::objects::{Method, Parameter, PythonClass};

static PARAMETER_SEPARATOR: &str = "||";

pub fn parse_method_parameter(parameters: Vec<String>) -> Vec<Parameter> {
    let mut parsed_parameters = Vec::new();
    for parameter in parameters.iter() {
        let values = parameter
            .split(PARAMETER_SEPARATOR)
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        if values.len() != 2 {
            continue;
        }
        let (param_name, param_annotation) = (values[0], values[1]);
        parsed_parameters.push(Parameter::new(
            param_name.to_string(),
            param_annotation.to_string(),
        ));
    }

    parsed_parameters
}

pub fn parse_method(methods_data: Vec<(String, String, String)>) -> Vec<Method> {
    let mut parsed_methods = Vec::new();
    for method in methods_data.iter() {
        let (method_name, method_parameters, method_output) =
            (method.0.clone(), method.1.clone(), method.2.clone());
        let raw_parameters = method_parameters
            .split(",")
            .map(|x| x.to_string())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>();
        let parameters = parse_method_parameter(raw_parameters);
        parsed_methods.push(Method {
            name: method_name.to_string(),
            parameters,
            output: method_output,
        });
    }

    parsed_methods
}

pub fn parse_class(
    name: String,
    methods: Vec<(String, String, String)>,
    docstring: String,
    inheritance: Vec<String>,
) -> PythonClass {
    let methods = parse_method(methods);
    PythonClass {
        name,
        methods,
        docstring,
        inheritance,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_method_params_ok() {
        let raw_parameters = vec!["self||str".to_string(), "cls||int".to_string()];
        let parsed_parameters = super::parse_method_parameter(raw_parameters);
        let expected_parameters = vec![
            super::Parameter::new("self".to_string(), "str".to_string()),
            super::Parameter::new("cls".to_string(), "int".to_string()),
        ];
        assert_eq!(parsed_parameters, expected_parameters);
    }

    #[test]
    fn test_parse_method_params_invalid_separator() {
        let raw_parameters = vec!["self||str".to_string(), "cls|int".to_string()];
        let parsed_parameters = super::parse_method_parameter(raw_parameters);
        let expected_parameters =
            vec![super::Parameter::new("self".to_string(), "str".to_string())];
        assert_eq!(parsed_parameters, expected_parameters);
    }

    #[test]
    fn test_parse_method_params_no_type() {
        let raw_parameters = vec!["self||str".to_string(), "xvalue||".to_string()];
        let parsed_parameters = super::parse_method_parameter(raw_parameters);
        let expected_parameters =
            vec![super::Parameter::new("self".to_string(), "str".to_string())];
        assert_eq!(parsed_parameters, expected_parameters);
    }

    #[test]
    fn test_parse_method_params_no_params() {
        let raw_parameters = vec!["".to_string()];
        let parsed_parameters = super::parse_method_parameter(raw_parameters);
        let expected_parameters = vec![];
        assert_eq!(parsed_parameters, expected_parameters);
    }

    #[test]
    fn test_parse_method_ok() {
        let raw_methods = vec![
            (
                "_this_method".to_string(),
                "self||None,xvalue||int".to_string(),
                "None".to_string(),
            ),
            (
                "another_method".to_string(),
                "self||None".to_string(),
                "str".to_string(),
            ),
        ];
        let parsed_methods = super::parse_method(raw_methods);
        let expected_methods = vec![
            super::Method {
                name: "_this_method".to_string(),
                parameters: vec![
                    super::Parameter::new("self".to_string(), "Self".to_string()),
                    super::Parameter::new("xvalue".to_string(), "int".to_string()),
                ],
                output: "None".to_string(),
            },
            super::Method {
                name: "another_method".to_string(),
                parameters: vec![super::Parameter::new(
                    "self".to_string(),
                    "Self".to_string(),
                )],
                output: "str".to_string(),
            },
        ];
        assert_eq!(parsed_methods, expected_methods);
    }

    #[test]
    fn test_parse_method_with_wrong_parameters() {
        let raw_methods = vec![
            (
                "_this_method".to_string(),
                "self||None,xvalue||ThisIsAClass".to_string(),
                "None".to_string(),
            ),
            (
                "another_method".to_string(),
                "self||None".to_string(),
                "str".to_string(),
            ),
            (
                "wrong_method".to_string(),
                "self||None,xvalue".to_string(),
                "str".to_string(),
            ),
        ];
        let parsed_methods = super::parse_method(raw_methods);
        let expected_methods = vec![
            super::Method {
                name: "_this_method".to_string(),
                parameters: vec![
                    super::Parameter::new("self".to_string(), "Self".to_string()),
                    super::Parameter::new("xvalue".to_string(), "ThisIsAClass".to_string()),
                ],
                output: "None".to_string(),
            },
            super::Method {
                name: "another_method".to_string(),
                parameters: vec![super::Parameter::new(
                    "self".to_string(),
                    "Self".to_string(),
                )],
                output: "str".to_string(),
            },
            super::Method {
                name: "wrong_method".to_string(),
                parameters: vec![super::Parameter::new(
                    "self".to_string(),
                    "Self".to_string(),
                )],
                output: "str".to_string(),
            },
        ];
        assert_eq!(parsed_methods, expected_methods);
    }

    #[test]
    fn test_parse_python_class() {
        let raw_methods = vec![
            (
                "_this_method".to_string(),
                "self||None,xvalue||int".to_string(),
                "None".to_string(),
            ),
            (
                "another_method".to_string(),
                "self||None".to_string(),
                "str".to_string(),
            ),
        ];
        let parsed_class = super::parse_class(
            "MyClass".to_string(),
            raw_methods,
            "This is a docstring".to_string(),
            vec!["MyParentClass".to_string()],
        );
        let expected_class = super::PythonClass {
            name: "MyClass".to_string(),
            methods: vec![
                super::Method {
                    name: "_this_method".to_string(),
                    parameters: vec![
                        super::Parameter::new("self".to_string(), "Self".to_string()),
                        super::Parameter::new("xvalue".to_string(), "int".to_string()),
                    ],
                    output: "None".to_string(),
                },
                super::Method {
                    name: "another_method".to_string(),
                    parameters: vec![super::Parameter::new(
                        "self".to_string(),
                        "Self".to_string(),
                    )],
                    output: "str".to_string(),
                },
            ],
            docstring: "This is a docstring".to_string(),
            inheritance: vec!["MyParentClass".to_string()],
        };
        assert_eq!(parsed_class, expected_class);
    }
}
