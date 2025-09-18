pub mod timestamp_serialize {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// 序列化 DateTime<Utc> 为 Unix 时间戳
    pub fn create_at_serialize<S>(
        create_at: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(create_at.timestamp())
    }

    /// 反序列化 Unix 时间戳为 DateTime<Utc>
    pub fn create_at_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts: i64 = Deserialize::deserialize(deserializer)?;
        let naive = NaiveDateTime::from_timestamp_opt(ts, 0)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid timestamp: {}", ts)))?;
        Ok(DateTime::<Utc>::from_utc(naive, Utc))
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Log {
        pub id: u64,
        pub message: String,
        #[serde(
            serialize_with = "create_at_serialize",
            deserialize_with = "create_at_deserialize"
        )]
        pub create_at: DateTime<Utc>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use timestamp_serialize::*;

    #[test]
    fn is_work() {
        // 解析为 NaiveDateTime 再转换为 UTC，保证时间戳一致
        let naive =
            NaiveDateTime::parse_from_str("2025-09-18 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let create_at = DateTime::<Utc>::from_utc(naive, Utc);

        let log = Log {
            id: 1,
            message: "test log".to_string(),
            create_at,
        };

        // 序列化
        let json = serde_json::to_string_pretty(&log).unwrap();
        println!("serialized log: {}", json);

        // 断言 JSON 中包含正确的时间戳
        let ts = log.create_at.timestamp();
        assert!(json.contains(&ts.to_string()));

        // 反序列化
        let decoded: Log = serde_json::from_str(&json).unwrap();
        println!("decoded log: {:#?}", decoded);

        assert_eq!(decoded.create_at, log.create_at);
    }
}
