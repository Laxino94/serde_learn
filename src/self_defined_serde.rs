/**
 * pub trait Serialize {
 *   serialize<S> (&self,serializer: S) -> Result<S::Ok, S::Error>
 *   where S: Serializer;
 * }
 * pub trait deserialize {
 *   deserialize<D> (deserializer: D) -> Result<Self, D::Error>
 *   where D: Deserializer<'de>;
 * }
*/
pub mod serde_customized {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize_hobbies<S>(hobbies: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(hobbies.join(", ").as_str())
    }

    pub fn deserialize_hobbies<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let v: Vec<&str> = s.split(",").collect();
        let mut arr: Vec<String> = Vec::new();
        for val in v {
            arr.push(val.trim().to_string());
        }
        Ok(arr)
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Person {
        pub id: i64,
        pub name: String,
        pub lang: String,
        pub is_married: bool,
        #[serde(
            serialize_with = "serialize_hobbies",
            deserialize_with = "deserialize_hobbies"
        )]
        pub hobbies: Vec<String>,
        pub address: Address,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Address {
        pub city: String,
        pub street: String,
        pub post_code: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_customized::*;

    #[test]
    fn is_work() {
        let p = Person {
            id: 2113324,
            name: "James Hues".to_string(),
            lang: "Java".to_string(),
            is_married: true,
            hobbies: vec!["reading".to_string(), "music".to_string()],
            address: Address {
                city: "Beijing".to_string(),
                street: "Chaoyang Road".to_string(),
                post_code: "100011".to_string(),
            },
        };
        let s = serde_json::to_string_pretty(&p).unwrap();
        println!("person encode to string: {}", s);
        let p: Person = serde_json::from_str(&s).unwrap();
        println!("person decode from string: {:#?}", p);
        println!(
            "person id: {}, name: {}, lang: {}, is_merried: {}, hobbies: {:?}, address: {:?}",
            p.id, p.name, p.lang, p.is_married, p.hobbies, p.address
        )
    }
}
