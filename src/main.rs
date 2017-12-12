#[macro_use]
extern crate nickel;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use std::env;
use std::net::*;
use std::fmt;
use std::thread;
use std::path::Path;

use nickel::{Nickel, Router, HttpRouter, StaticFilesHandler};

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

fn main() {

    let conn_string:String = match env::var("POSTGRESQL_ADDON_URI") {
        Ok(val) => val,
        Err(_) => "postgres://cdvv:cdvv@localhost:5432/cdvv".to_string()
    };

    println!("{}", conn_string);

    let manager = PostgresConnectionManager::new(conn_string, TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();
    let cwd:String = env::var("PWD").unwrap();
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("public/"));
    server.utilize(router());

    server.listen("0.0.0.0:8080");
}

fn router() -> nickel::Router {

    let mut router = Nickel::router();

    router.get("/persons/:id", middleware! { |request|
        let id = request.param("id").unwrap();
        format!("read person '{}'", id)
    });

    router
}
