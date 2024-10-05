#[macro_use] extern crate rocket;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

mod utils;
mod cors;
mod callback_result;
mod validation;

mod desk;
mod dish;
mod species;

mod request;

#[get("/")]
fn index() -> String {
    "demeter at your service".to_string()
}

#[launch]
async fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8007)))
        .manage(SqlitePool::connect_with(SqliteConnectOptions::new()
            .filename("db")
        ).await.unwrap())
        .attach(cors::CORS)
        .mount("/", routes![index])

        // admin permissions
        .mount("/table/create", routes![desk::create])
        .mount("/table/delete", routes![desk::delete])
        .mount("/table/fetch", routes![desk::fetch])
        .mount("/table/fetch_all", routes![desk::fetch_all])

        .mount("/species/create", routes![species::create])
        .mount("/species/delete", routes![species::delete])
        .mount("/species/edit", routes![species::edit])

        .mount("/dish/create", routes![dish::create])
        .mount("/dish/delete", routes![dish::delete])
        .mount("/dish/edit", routes![dish::edit])

        // table permissions
        .mount("/request/create", routes![request::create])
        .mount("/request/delete", routes![request::delete])
        .mount("/request/edit", routes![request::edit])
        .mount("/request/fetch", routes![request::fetch])

        // no permission
        .mount("/species/fetch", routes![species::fetch])
        .mount("/species/fetch_all", routes![species::fetch_all])
        .mount("/species/fetch_by_name", routes![species::fetch_by_name])

        .mount("/dish/fetch", routes![dish::fetch])
        .mount("/dish/fetch_all", routes![dish::fetch_all])
}
