#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{env, process};

#[derive(Serialize)]
struct NoContext {}

#[launch]
fn rocket() -> rocket::Rocket {
    let secret = env::var("SECRET").unwrap_or_else(|_| {
        println!("Secret is not set!");
        process::exit(1);
    });

    rocket::ignite()
        .manage(secret)
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, live, events])
        .mount("/ru", routes![ru_index])
        .register(catchers![not_found])
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[get("/live")]
pub fn live() -> Template {
    Template::render("live", NoContext {})
}

#[get("/events")]
pub fn events() -> Template {
    Template::render("events", NoContext {})
}

#[get("/")]
pub fn ru_index() -> Template {
    Template::render("ru_index", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}
