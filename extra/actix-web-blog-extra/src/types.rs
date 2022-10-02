use diesel::backend::Backend;
use diesel::backend::RawValue;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::{Integer, Text};
use diesel::*;
use serde::{Deserialize, Serialize};

pub mod post {
    use super::*;

    #[derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        AsExpression,
        FromSqlRow,
    )]
    #[serde(into = "i32")]
    #[diesel(sql_type = Integer)]
    pub struct Id(i32);

    impl From<Id> for i32 {
        fn from(id: Id) -> Self {
            id.0
        }
    }

    impl<DB> FromSql<Integer, DB> for Id
    where
        DB: Backend,
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(
            bytes: RawValue<DB>,
        ) -> deserialize::Result<Self> {
            i32::from_sql(bytes).map(Self)
        }
    }

    impl<DB> ToSql<Integer, DB> for Id
    where
        DB: Backend,
        i32: ToSql<Integer, DB>,
    {
        fn to_sql<'a>(
            &'a self,
            out: &mut Output<'a, '_, DB>,
        ) -> serialize::Result {
            i32::to_sql(&self.0, out)
        }
    }

    #[derive(
        Debug,
        Clone,
        Serialize,
        //Deserialize,
        AsExpression,
        FromSqlRow,
    )]
    #[serde(into = "String")]
    #[diesel(sql_type = Text)]
    pub struct Title(String);

    impl From<Title> for String {
        fn from(id: Title) -> Self {
            id.0
        }
    }

    /// デシリアライズ時のバリデーション処理
    impl<'de> Deserialize<'de> for Title {
        fn deserialize<D>(
            deserializer: D,
        ) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer).and_then(|s| {
                if !s.is_empty() && s.len() <= 100 {
                    Ok(Self(s))
                } else {
                    Err(serde::de::Error::custom(
                        "string length is 0 or too long.",
                    ))
                }
            })
        }
    }

    impl<DB> FromSql<Text, DB> for Title
    where
        DB: Backend,
        String: FromSql<Text, DB>,
    {
        fn from_sql(
            bytes: RawValue<DB>,
        ) -> deserialize::Result<Self> {
            String::from_sql(bytes).map(Self)
        }
    }

    impl<DB> ToSql<Text, DB> for Title
    where
        DB: Backend,
        String: ToSql<Text, DB>,
    {
        fn to_sql<'a>(
            &'a self,
            out: &mut Output<'a, '_, DB>,
        ) -> serialize::Result {
            String::to_sql(&self.0, out)
        }
    }

    #[derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        AsExpression,
        FromSqlRow,
    )]
    #[serde(into = "String")]
    #[diesel(sql_type = Text)]
    pub struct Body(String);

    impl From<Body> for String {
        fn from(id: Body) -> Self {
            id.0
        }
    }

    impl<DB> FromSql<Text, DB> for Body
    where
        DB: Backend,
        String: FromSql<Text, DB>,
    {
        fn from_sql(
            bytes: RawValue<DB>,
        ) -> deserialize::Result<Self> {
            String::from_sql(bytes).map(Self)
        }
    }

    impl<DB> ToSql<Text, DB> for Body
    where
        DB: Backend,
        String: ToSql<Text, DB>,
    {
        fn to_sql<'a>(
            &'a self,
            out: &mut Output<'a, '_, DB>,
        ) -> serialize::Result {
            String::to_sql(&self.0, out)
        }
    }
}
