use sha2::{Digest, Sha256};
use sqlx::{Pool, Sqlite};

use crate::utils::{ValueInt, ValueString};

#[derive(Debug, Clone)]
pub struct Validation;
impl Validation {
    pub async fn table_hash(db: &Pool<Sqlite>, id: String) -> Option<String> {
        // checks for table hashes
        // if hash matches no table, return none
        // matches, return table name

        let hashes = sqlx::query_as::<_, ValueString>("select name from desk;")
            .fetch_all(db)
            .await
            .unwrap();

        for h in hashes {
            let mut hasher = Sha256::new();
            hasher.update(h.0.clone());
            let result = hasher.finalize()[..].to_vec().iter().map(|x| format!("{:x}", x)).collect::<String>();

            if result == id {
                return Some(h.0);
            }
        }

        None
    }

    pub async fn verify_admin(db: &Pool<Sqlite>, id: String, secret: String) -> bool {
        // true if verified

        sqlx::query_as::<_, ValueInt>("select count(*) from admin where id = $1 and secret = $2;")
            .bind(id)
            .bind(secret)
            .fetch_one(db)
            .await
            .unwrap()
            .0 > 0
    }
}
