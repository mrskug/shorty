#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod cache;
mod database;
pub mod models;
pub mod schema;
mod shortener;

use cache::Cache;
use database::Database;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::sync::RwLock;

#[derive(FromForm)]
struct Url {
    url: String,
}

#[derive(FromForm)]
struct NamedUrl {
    url: String,
    name: String,
}

#[get("/<hash>")]
fn lookup(repo: State<RwLock<Database>>, hash: &RawStr) -> Result<Redirect, String> {
    match repo.read().unwrap().lookup(hash) {
        Some(url) => Ok(Redirect::permanent(url.to_string())),
        _ => Err("Requested ID was not found.".to_string()),
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<Database>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url = url_form.url;
    let mut repo = repo.write().unwrap();
    let hash = repo.store(&url);
    Ok(hash.to_string())
}

#[post("/named", data = "<url_form>")]
fn named_url(repo: State<RwLock<Database>>, url_form: Form<NamedUrl>) -> Result<String, String> {
    let ref url = url_form.url;
    let ref name = url_form.name;
    let mut repo = repo.write().unwrap();
    let named = repo.store_named(&url, &name);
    Ok(named.to_string())
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(Database::new()))
        .mount("/", routes![lookup, shorten, named_url])
        .launch();
}
