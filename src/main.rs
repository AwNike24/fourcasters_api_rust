#[macro_use] extern crate rocket;

mod db;
use db::establish_connection;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .manage(establish_connection().await)
        .mount("/", routes![index])
        .launch()
        .await;
}
