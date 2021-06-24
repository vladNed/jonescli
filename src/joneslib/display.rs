use super::objects;

pub fn output_class(python_class: &objects::PythonClass) {
    println!("{}", python_class);

    for method in python_class.methods.iter() {
        println!("{}", method);
        for parameter in method.parameters.iter() {
            println!("{}", parameter);
        }
    }
}