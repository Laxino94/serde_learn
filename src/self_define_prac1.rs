mod user_serialize {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize_pwd<S>(pwd: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let masked = "*".repeat(pwd.len());
        serializer.serialize_str(&masked)
    }

    pub fn deserialize_pwd<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s)
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub username: String,
        #[serde(serialize_with = "serialize_pwd", deserialize_with = "deserialize_pwd")]
        pub password: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use user_serialize::*;

    #[test]
    fn is_work() {
        let user = User {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
        };
        let s = serde_json::to_string_pretty(&user).unwrap();
        println!("serialized result: \n{s}");
        assert!(s.contains("*"));

        let decode: User =
            serde_json::from_str(r#"{"username":"alice","password":"12345"}"#).unwrap();
        println!("user decode from string: {:#?}", decode);
        assert_eq!(decode.password, "12345")
    }
}
