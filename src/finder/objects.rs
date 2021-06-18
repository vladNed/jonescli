use super::extractors;

pub struct Parameter{
    name: String,
    static_type: String
}
impl Parameter {
    pub fn new(name: String, static_type: String) -> Self{
        Parameter {
            name,
            static_type
        }
    }
}

pub struct Method{
    name: String,
    parameters: Vec<Parameter>
}
impl Method {
    pub fn new(method_header: &String) -> Self {
        Method {
            name: extractors::extract_method_name(method_header),
            parameters: extractors::extract_parameters(method_header)
        }
    }

}

pub struct PythonClass{
    name: String,
    methods: Vec<Method>
}
impl PythonClass {
    pub fn new(class_code: Vec<String>, name: String) -> Self {
        PythonClass {
            name: name,
            methods: extractors::extract_methods(class_code)
        }
    }
}