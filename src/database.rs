extern crate dotenv;

use crate::cache::Cache;
use crate::models::ShUrl;
use crate::schema::urls::dsl::*;
use crate::shortener::Shortener;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct Database {
    shortener: Shortener,
}

impl Database {
    pub fn new() -> Database {
        Database {
            shortener: Shortener::new(),
        }
    }
}

impl Cache for Database {
    fn store(&mut self, uri: &str) -> String {
        let conn = db_conn();
        let id_ = urls
            .select(id)
            .order(id.desc())
            .limit(1)
            .load::<i32>(&conn)
            .expect("fail");
        let short = if id_.len() > 0 {
            let u64_id = if id_[0] < 0 {
                None
            } else {
                Some(id_[0] as u64)
            };
            self.shortener.get_short(u64_id.unwrap())
        } else {
            self.shortener.get_short(0)
        };
        insert_into(urls)
            .values((hash.eq(short.to_string()), url.eq(uri.to_string())))
            .execute(&conn)
            .expect("Failed to insert hashed uri");
        short
    }

    fn store_named(&mut self, uri: &str, name: &str) -> String {
        let conn = db_conn();
        insert_into(urls)
            .values((hash.eq(name.to_string()), url.eq(uri.to_string())))
            .execute(&conn)
            .expect("Failed to insert named uri");
        name.to_string()
    }

    fn lookup(&self, short: &str) -> Option<String> {
        let conn = db_conn();
        let result = urls
            .filter(hash.eq(short))
            .limit(1)
            .load::<ShUrl>(&conn)
            .expect("fail");
        if result.len() > 0 {
            Some(result[0].url.to_string())
        } else {
            None
        }
    }
}

fn db_conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
