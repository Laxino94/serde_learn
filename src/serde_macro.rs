pub mod serde_macro {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub id: u64,
        pub username: String,
        pub nick: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json; // macro
    use serde_macro::*;

    #[test]
    fn is_work() {
        let user = User {
            id: 1,
            username: "test_user".to_string(),
            nick: "test_nick".to_string(),
        };
        let res = json!(user);
        println!(
            "id: {}, nick: {}, username: {}",
            res["id"], res["nick"], res["username"]
        );

        println!("res josn encode: {}\n", res.to_string());
    }
}
