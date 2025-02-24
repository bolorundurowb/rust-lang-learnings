

pub trait JsonSerializable {
    fn to_json(&self) -> String;
}

use json_serializable_lib::JsonSerializable;

// #[derive(JsonSerializable)]
// struct MyJsonSerializable {
//     num: f32,
//     string: String,
// }

#[derive(JsonSerializable)]
struct MyJsonSerializableTwo ;

fn main() {
    // let my_data = MyJsonSerializable {
    //     num: 12.34f32,
    //     string: String::from("Hello world"),
    // };
    let my_data_two = MyJsonSerializableTwo {};
    println!("My Data: {}", my_data_two.to_json());
}
