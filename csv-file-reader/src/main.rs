use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct MyData {
    username: String,
    first_name: String,
    last_name: String,
    identifier: String,
}

impl MyData {
    fn init(field_map: HashMap<&str, String>) -> MyData {
        let username = field_map.get("Username").unwrap().to_string();
        let first_name = field_map.get("First name").unwrap().to_string();
        let last_name = field_map.get("Last name").unwrap().to_string();
        let identifier = field_map.get("Identifier").unwrap().to_string();

        MyData {
            username,
            first_name,
            last_name,
            identifier,
        }
    }
}

fn main() {
    let file_path = "C:\\Users\\bolorundurowb\\Downloads\\username.csv";

    let raw_contents = fs::read_to_string(file_path).expect("File could not be read");
    let content_rows = raw_contents
        .split('\n')
        .collect::<Vec<&str>>();
    let headers = split_row(content_rows[0]);
    let mut results: Vec<MyData> = vec![];

    for row in content_rows.iter().skip(1) {
        if !row.trim().is_empty() {
            let cells = split_row(row);
            let mut hash_map = HashMap::new();

            for (index, cell) in cells.iter().enumerate() {
                hash_map.insert(headers[index], cell.to_string());
            }

            let my_data = MyData::init(hash_map);
            results.push(my_data);
        }
    }

    println!("{:?}", &results);
}

fn split_row(row: &str) -> Vec<&str> {
    row.split(';').map(|x| x.trim()).collect::<Vec<&str>>()
}
