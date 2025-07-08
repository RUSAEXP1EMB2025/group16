pub mod error;

use color_eyre::eyre::{self, Context as _};
use error::AtmosdictError;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{collections::HashSet, str::FromStr as _};

#[derive(sqlx::FromRow)]
pub struct Wordlist {
    pub word: String,
    pub is_positive: bool,
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

    pub fn from_pool(pool: SqlitePool) -> Self {
        Atmosdict { pool }
    }

    pub async fn get_all(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>("SELECT word, is_positive from atmoswords")
            .fetch_all(&self.pool)
            .await?;
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Ok(wordlist)
    }

    pub async fn get_positive(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>(
            "SELECT word, is_positive from atmoswords WHERE is_positive = true",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Ok(wordlist)
    }

    pub async fn get_negative(&self) -> Result<HashSet<String>, AtmosdictError> {
        let words = sqlx::query_as::<_, Wordlist>(
            "SELECT word, is_positive from atmoswords WHERE is_positive = false",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();
        let wordlist = words
            .into_iter()
            .map(|wordlist| wordlist.word)
            .collect::<HashSet<_>>();
        Ok(wordlist)
    }
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;

    use crate::{Atmosdict, error::AtmosdictError};

    #[sqlx::test(migrations = "../../../db/migrations", fixtures("atmoswords.sql"))]
    fn test_get_all(pool: SqlitePool) -> Result<(), AtmosdictError> {
        let atmosdict = Atmosdict::from_pool(pool);
        let atmoswords = atmosdict.get_all().await?;
        dbg!(atmoswords);
        Ok(())
    }

    #[sqlx::test(migrations = "../../../db/migrations", fixtures("atmoswords.sql"))]
    fn test_get_posositive(pool: SqlitePool) -> Result<(), AtmosdictError> {
        let atmosdict = Atmosdict::from_pool(pool);
        let atmoswords = atmosdict.get_positive().await?;
        dbg!(atmoswords);
        Ok(())
    }

    #[sqlx::test(migrations = "../../../db/migrations", fixtures("atmoswords.sql"))]
    fn test_get_negative(pool: SqlitePool) -> Result<(), AtmosdictError> {
        let atmosdict = Atmosdict::from_pool(pool);
        let atmoswords = atmosdict.get_negative().await?;
        dbg!(atmoswords);
        Ok(())
    }
}
