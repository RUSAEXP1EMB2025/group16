//! これはポジティブ/ネガティブな単語を操作するライブラリである。

pub mod error;

use color_eyre::eyre::{self, Context as _, Ok};
use error::AtmosdictError;
use sqlx::{SqlitePool, prelude::FromRow, sqlite::SqliteConnectOptions};
use std::{collections::HashSet, str::FromStr as _};

#[derive(sqlx::FromRow)]
pub struct Wordlist {
    word: String,
    is_positive: bool,
}

#[derive(Clone, Debug)]
pub struct Atmosdict {
    pool: SqlitePool,
}

impl AsRef<Atmosdict> for Atmosdict {
    fn as_ref(&self) -> &Atmosdict {
        self
    }
}

impl Atmosdict {
    pub async fn new(path: &str) -> Result<Self, eyre::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Atmosdict { pool })
    }

    pub async fn get_all(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>("SELECT word, is_positive from WORDLIST")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Result::<_, AtmosdictError>::Ok(wordlist)
    }

    pub async fn get_positive(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>("SELECT word, is_positive from WORDLIST WHERE is_positive = true")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Result::<_, AtmosdictError>::Ok(wordlist)
    }

    pub async fn get_negative(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>("SELECT word, is_positive from WORDLIST WHERE is_positive = false")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Result::<_, AtmosdictError>::Ok(wordlist)
    }
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "../../../db/migrations", fixtures("atmoswords.sql"))]
    fn test_get_all(pool: SqlitePool) -> Result<(), sqlx::Error> {
        Ok(())
    }

    #[sqlx::test(migrations = "../../../db/migrations", fixtures("atmoswords.sql"))]
    fn test_get_pos_neg(pool: SqlitePool) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
