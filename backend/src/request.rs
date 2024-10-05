use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, desk::Desk, utils::ValueString};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: i32,
    pub desk: String,
    pub dish: i32,
    pub variant: i32,
    pub size: i32,
    pub comment: String,
    pub state: i32
    // 0 -> pending
    // 1 -> in kitchen
    // 2 -> completed
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

    pub async fn create(db: &Pool<Sqlite>, desk: String, dish: i32, variant: i32, size: i32, comment: String, state: i32) -> Result {
        // CREATE TABLE request(id integer primary key autoincrement, desk varchar, dish int, variant int, size int, comment varchar, state int);
        if !Request::check_variant(db, dish, variant).await {
            return Result::VariantDoesntExist;
        }

        if !Request::check_size(db, dish, size).await {
            return Result::SizeDoesntExist;
        }

        sqlx::query("insert into request(desk, dish, variant, size, comment, state) values($1, $2, $3, $4, $5, $6);")
            .bind(desk)
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

    pub async fn edit(db: &Pool<Sqlite>, request_id: i32, variant: i32, size: i32, comment: String, state: i32) -> Result {
        let dish;
        match Request::fetch(&db, request_id).await {
            Some(r) => {
                dish = r;
            },
            None => {
                return Result::DoesntExist;
            }
        }

        if !Request::check_variant(db, dish.id, variant).await {
            return Result::VariantDoesntExist;
        }

        if !Request::check_size(db, dish.id, size).await {
            return Result::SizeDoesntExist;
        }

        sqlx::query("update request set variant = $1, size = $2, comment = $3, state = $4 where id = $5;")
            .bind(variant)
            .bind(size)
            .bind(comment)
            .bind(state)
            .bind(dish.id)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }

    pub async fn fetch(db: &Pool<Sqlite>, request_id: i32) -> Option<Request> {
        match sqlx::query_as::<_, Request>("select * from request where id = $1;")
        .bind(request_id)
        .fetch_one(db)
        .await {
            Ok(r) => Some(r),
            Err(e) => {
                println!("request.rs; fetch({request_id}); error: {e}");
                None
            }
        }
    }

    pub async fn delete(db: &Pool<Sqlite>, request_id: i32) -> Result {
        match Request::fetch(db, request_id).await {
            Some(_) => {
                sqlx::query("delete from request where id = $1;")
                    .bind(request_id)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            },
            None => Result::DoesntExist
        }
    }
}

#[post("/<dish>/<variant>/<size>/<comment>/<state>", data="<table>")]
pub async fn create(db: &State<Pool<Sqlite>>, table: String, dish: i32, variant: i32, size: i32, comment: String, state: i32) -> String {
    let db = db.inner();
    match Desk::fetch(db, &table).await {
        Some(d) => {
            Request::create(&db, d.name, dish, variant, size, comment, state).await.to_string()
        },
        None => Result::NoTable.to_string()
    }
}

#[post("/<request_id>/<variant>/<size>/<comment>/<state>", data="<table>")]
pub async fn edit(db: &State<Pool<Sqlite>>, table: String, request_id: i32, variant: i32, size: i32, comment: String, state: i32) -> String {
    let db = db.inner();
    match Desk::fetch(db, &table).await {
        Some(_) => {
            Request::edit(db, request_id, variant, size, comment, state).await.to_string()
        },
        None => Result::NoTable.to_string()
    }
}

#[post("/<request_id>", data="<table>")]
pub async fn fetch(db: &State<Pool<Sqlite>>, table: String, request_id: i32) -> String {
    let db = db.inner();
    match Desk::fetch(db, &table).await {
        Some(_) => {
            serde_json::to_string(&Request::fetch(db, request_id).await).unwrap()
        },
        None => Result::NoTable.to_string()
    }
}

#[post("/<request_id>", data="<table>")]
pub async fn delete(db: &State<Pool<Sqlite>>, table: String, request_id: i32) -> String {
    let db = db.inner();
    match Desk::fetch(db, &table).await {
        Some(_) => {
            Request::delete(db, request_id).await.to_string()
        },
        None => Result::NoTable.to_string()
    }
}


