use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, desk::Desk, request::Request, utils};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: i32, 
    pub start: i32,
    pub end: i32,

    pub desk: String,
    pub state: i32,
    // 1 -> open
    // 0 -> closed
}
impl Session {
    pub async fn create(db: &Pool<Sqlite>, start: i32, end: i32, desk: String, state: i32) -> Result {
        // TODO : check for desk existance

        sqlx::query("insert into session(start, end, desk, state) values($1, $2, $3, $4);")
            .bind(start)
            .bind(end)
            .bind(desk)
            .bind(state)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }

    pub async fn edit(db: &Pool<Sqlite>, session: i32, start: i32, end: i32, desk: String, state: i32) -> Result {
        match Session::fetch(db, session).await {
            Some(s) => {
                sqlx::query("update session set start = $1, end = $2, desk = $3, state = $4 where id = $5;")
                    .bind(start)
                    .bind(end)
                    .bind(desk)
                    .bind(state)
                    .bind(s.id)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            },
            None => Result::DoesntExist
        }
    }

    pub async fn fetch(db: &Pool<Sqlite>, session: i32) -> Option<Session> {
        match sqlx::query_as::<_, Session>("select * from session where id = $1;")
        .bind(session)
        .fetch_one(db)
        .await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("session.rs; fetch({session}); error : {e}");
                None
            }
        }
    }

    pub async fn fetch_only_open(db: &Pool<Sqlite>, session: i32) -> Option<Session> {
        match sqlx::query_as::<_, Session>("select * from session where id = $1 and state = 1;")
        .bind(session)
        .fetch_one(db)
        .await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("session.rs; fetch({session}); error : {e}");
                None
            }
        }
    }

    pub async fn fetch_all(db: &Pool<Sqlite>) -> Vec<Session> {
        sqlx::query_as("select * from session;")
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn fetch_all_open(db: &Pool<Sqlite>) -> Vec<Session> {
        sqlx::query_as("select * from session where state = 1;")
            .fetch_all(db)
            .await
            .unwrap()
    }

    // better not delete anything
    // pub async fn delete(db: &Pool<Sqlite>, session: i32) -> Result {
    //     match Session::fetch(db, session).await {
    //         Some(_) => {
    //             sqlx::query("delete from session where id = $1;")
    //                 .bind(session)
    //                 .execute(db)
    //                 .await
    //                 .unwrap();

    //             Result::Success
    //         },
    //         None => Result::DoesntExist
    //     }
    // }

    pub async fn fetch_all_requests(db: &Pool<Sqlite>, session: i32) -> Vec<Request> {
        sqlx::query_as("select * from request where session = $1;")
            .bind(session)
            .fetch_all(db)
            .await
            .unwrap()
    }
}

#[get("/<desk>")]
pub async fn start(db: &State<Pool<Sqlite>>, desk: String) -> String {
    let desk = urlencoding::decode(&desk).unwrap().to_string();
    match Desk::fetch(db, &desk).await {
        Some(_) => {
            match Desk::fetch_open_session(db, &desk).await {
                Some(_) => Result::TableOccupied.to_string(),
                None => Session::create(db.inner(), utils::get_time(), -1, desk, 1).await.to_string()
            }
        },
        None => Result::DoesntExist.to_string()
    }
}

#[get("/<session>")]
pub async fn end(db: &State<Pool<Sqlite>>, session: i32) -> String {
    let db = db.inner();
    match Session::fetch_only_open(db, session).await {
        Some(s) => {
            Session::edit(db, s.id, s.start, utils::get_time(), s.desk, 0).await.to_string()
        },
        None => {
            Result::DoesntExist.to_string()
        }
    }
}

#[get("/<desk>")]
pub async fn end_by_desk(db: &State<Pool<Sqlite>>, desk: String) -> String {
    let db = db.inner();
    let desk = urlencoding::decode(&desk).unwrap().to_string();
    match Desk::fetch(db, &desk).await {
        Some(_) => {
            match Desk::fetch_open_session(db, &desk).await {
                Some(s) => Session::edit(db, s.id, s.start, utils::get_time(), desk, 0).await.to_string(),
                None => Result::TableUnoccupied.to_string()
            }
        },
        None => Result::DoesntExist.to_string()
    }
}


#[get("/<session>/<to>")]
pub async fn change_desk(db: &State<Pool<Sqlite>>, session: i32, to: String) -> String {
    let db = db.inner();
    let to = urlencoding::decode(&to).unwrap().to_string();
    match Session::fetch_only_open(db, session).await {
        Some(s) => {
            Session::edit(db, s.id, s.start, s.end, to, s.state).await;

            Result::Success.to_string()
        },
        None => {
            Result::DoesntExist.to_string()
        }
    }
}

#[get("/<from>/<to>")]
pub async fn change_desk_by_desk(db: &State<Pool<Sqlite>>, from: String, to: String) -> String {
    let db = db.inner();
    let from = urlencoding::decode(&from).unwrap().to_string();
    let to = urlencoding::decode(&to).unwrap().to_string();
    match Desk::fetch_open_session(db, &from).await {
        // check if 'from' desk has a session
        Some(s) => {
            match Desk::fetch_open_session(db, &to).await {
                // check if 'to' desk does not have a session
                Some(_) => Result::TableOccupied.to_string(),
                None => Session::edit(db, s.id, s.start, s.end, to, s.state).await.to_string()
            }
        },
        None => {
            Result::DoesntExist.to_string()
        }
    }
}

#[get("/<session>")]
pub async fn fetch(db: &State<Pool<Sqlite>>, session: i32) -> String {
    serde_json::to_string(&Session::fetch(db.inner(), session).await).unwrap()
}

#[get("/")]
pub async fn fetch_open(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Session::fetch_all_open(db.inner()).await).unwrap()
}

#[get("/")]
pub async fn fetch_all(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Session::fetch_all(db.inner()).await).unwrap()
}

#[get("/<session>")]
pub async fn fetch_requests(db: &State<Pool<Sqlite>>, session: i32) -> String {
    serde_json::to_string(&Session::fetch_all_requests(db.inner(), session).await).unwrap()
}

