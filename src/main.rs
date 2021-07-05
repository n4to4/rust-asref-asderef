//fn main() {
//    let option_name: Option<String> = Some("Alice".to_owned());
//    match &option_name {
//        Some(name) => println!("Name is {}", name),
//        None => println!("No name provided"),
//    }
//    println!("{:?}", option_name);
//}

//fn try_greet(option_name: Option<&str>) {
//    match option_name {
//        Some(name) => println!("Name is {}", name),
//        None => println!("No name provided"),
//    }
//}

//fn greet(name: &str) {
//    println!("Name is {}", name);
//}

//fn main() {
//    let option_name: Option<String> = Some("Alice".to_owned());
//    //try_greet(option_name.as_ref().map(|s| s.as_str()));
//    //try_greet(option_name.as_deref());
//    option_name.as_deref().map(greet);
//    println!("{:?}", option_name);
//}

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    some_value: Option<String>,
}

const DEFAULT_VALUE: &str = "my-default-value";

fn main() {
    let mut file = std::fs::File::open("config.yaml").unwrap();
    let config: Config = serde_yaml::from_reader(&mut file).unwrap();
    let value = config.some_value.as_deref().unwrap_or(DEFAULT_VALUE);
    println!("value is {}", value);
}
