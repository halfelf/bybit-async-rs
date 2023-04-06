pub mod string_or_decimal {
    use rust_decimal::Decimal;
    use serde::{
        de::{self},
        Deserialize, Deserializer, Serializer,
    };
    use std::fmt::{self};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(Decimal),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(Decimal::MAX)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub mod string_or_decimal_opt {
    use rust_decimal::Decimal;
    use serde::{Deserializer, Serializer};
    use std::fmt::{self};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => super::string_or_decimal::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(super::string_or_decimal::deserialize(deserializer)?))
    }
}

pub mod string_or {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt, str::FromStr};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + Deserialize<'de>,
        T::Err: fmt::Display,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOr<T> {
            String(String),
            T(T),
        }

        match StringOr::deserialize(deserializer)? {
            StringOr::String(s) => s.parse().map_err(de::Error::custom),
            StringOr::T(t) => Ok(t),
        }
    }
}
