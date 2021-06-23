mod extractors;

fn main() {

    // TODO: This will become argumens
    let class_name = String::from("Human");

    // Generate python class
    let class = extractors::project_traversal(&"./testsdir".to_string(), &class_name);
    match class {
        Some(class) => println!("{:?}", class),
        None => println!("Found none")
    }
}
