use fake::faker::internet::en::Username;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::{FirstName, LastName};
use fake::{Dummy, Fake, Faker};
use std::fs;

#[derive(Debug, Dummy)]
struct MyData {
    #[dummy(faker = "Username()")]
    username: String,

    #[dummy(faker = "FirstName()")]
    first_name: String,

    #[dummy(faker = "LastName()")]
    last_name: String,

    #[dummy(faker = "Word()")]
    identifier: String,
}

impl MyData {
    fn to_string(self) -> String {
        format!(
            "{}; {}; {}; {}\n",
            self.username, self.first_name, self.last_name, self.identifier
        )
    }
}

fn main() {
    let mut output = format!(
        "{}; {}; {}; {}\n",
        "Username", "First Name", "Last Name", "Identifier"
    );

    for _ in 0..5 {
        let datum: MyData = Faker.fake();
        output.push_str(&datum.to_string());
    }

    fs::write("output.csv", output).expect("Failed writing output.csv");
    println!("CSV file generated successfully!");
}
