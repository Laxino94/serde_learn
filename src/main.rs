use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
fn main() {
    let mut m = HashMap::new();
    m.insert("id", 1);
    m.insert("postcode", 518000);
    // Convert HashMap to JSON string using serde_json::to_string method
    let s = serde_json::to_string(&m).unwrap();
    println!("json str: {}", s);

    // Convert JSON string back to HashMap using serde_json::from_str method
    let value: Value = serde_json::from_str(&s).unwrap();
    println!("id: {}, postcode: {}", value["id"], value["postcode"]);
    println!(
        "call as_i64 method for id: {}, postcode: {}",
        value["id"].as_i64().unwrap(),
        value["postcode"].as_i64().unwrap()
    );
    let p = Person {
        id: 2123375,
        name: "Laxino".to_string(),
        lang: "Rust".to_string(),
        is_married: false,
        hobbies: vec!["reading".to_string(), "musics".to_string()],
        address: Address {
            city: "Xi'an".to_string(),
            street: "Weiyang Road".to_string(),
            post_code: "710032".to_string(),
        },
    };

    println!("person: {:?}", p);
    let s = serde_json::to_string(&p).unwrap();
    println!("person encode to string: {}", s);
    let p: Value = serde_json::from_str(&s).unwrap();
    println!("person decode from string: {:?}", p);
    let result: serde_json::Result<Person> = serde_json::from_str(&s);
    match result {
        Ok(v) => println!(
            "person id: {}, name: {}, lang: {}, is_married: {}, hobbies: {:?}, address: {:?}",
            v.id, v.name, v.lang, v.is_married, v.hobbies, v.address
        ),
        Err(e) => println!("fail to parse with error: {e}"),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i64,
    name: String,
    lang: String,
    is_married: bool,
    hobbies: Vec<String>,
    address: Address,
}
#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    street: String,
    post_code: String,
}
