mod utils;

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {
    // println!("Hello, world!");

    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: u8,
    // }

    // let person = Person {
    //     name: String::from("Alice"),
    //     age: 25,
    // };

    // println!("{:?}", person);
    // println!("{}", person.name);
    // println!("{}", LANGUAGE);
    // let decimal = 65.4321_f32;
    // println!("{}", decimal);

    utils::logger::Info("Hello, world!".to_string());
    utils::logger::Warn("message can be truncated".to_string());
    utils::logger::Error("Oops, world!".to_string(), None);
}