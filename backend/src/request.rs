use rocket::State;
use sqlx::{Row, sqlite::SqliteRow, Pool, Sqlite};

use crate::{callback_result::Result, utils::ValueString};

pub struct Request {
    pub id: i32,
    pub session: i32,
    pub dish: i32,
    pub variant: i32,
    pub size: i32,
    pub comment: String,
    pub state: i32
}
impl Request {
    pub async fn check_variant(db: &Pool<Sqlite>, dish: i32, variant: i32) -> bool {
        let variants = sqlx::query_as::<_, ValueString>("select variants from dish where id = $1;")
            .bind(dish)
            .fetch_one(db)
            .await
            .unwrap().0;

        if variant < 0 {
            return false;
        }

        variants.split(',').count() as i32 > variant
    }

    pub async fn check_size(db: &Pool<Sqlite>, dish: i32, size: i32) -> bool {
        let sizes = sqlx::query_as::<_, ValueString>("select sizes from dish where id = $1;")
            .bind(dish)
            .fetch_one(db)
            .await
            .unwrap().0;

        if size < 0 {
            return false;
        }

        sizes.split(',').count() as i32 > size
    }

    pub async fn create(db: &Pool<Sqlite>, session: i32, dish: i32, variant: i32, size: i32, comment: String, state: i32) -> Result {
        // CREATE TABLE request(id integer primary key autoincrement, session int, dish int, variant int, size int, comment varchar, state int);
        if !Request::check_variant(db, dish, variant).await {
            return Result::VariantDoesntExist;
        }

        if !Request::check_size(db, dish, size).await {
            return Result::SizeDoesntExist;
        }

        sqlx::query("insert into request(session, dish, variant, size, comment, state) values($1, $2, $3, $4, $5, $6);")
            .bind(session)
            .bind(dish)
            .bind(variant)
            .bind(size)
            .bind(comment)
            .bind(state)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }
}

//  session: i32, dish: i32, variant: i32, size: i32, comment: String, state: i32)
// #[get("")]
// pub async fn create(db: &State<Pool<Sqlite>>) -> String {}

