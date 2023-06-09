mod api;
mod model;
mod util;

use std::sync::{Mutex, Arc};

use mysql::{prelude::*, Opts, OptsBuilder};

use actix_web::{web, App, HttpServer};
use r2d2::{Pool as R2D2Pool};
use r2d2_mysql::MySqlConnectionManager;
use crate::api::tag::get_tags;
use std::env;
use dotenv::dotenv;
use std::fs;


fn create_db_pool() -> R2D2Pool<MySqlConnectionManager> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
    let ops = Opts::from_url(&database_url).unwrap();
    let builder = OptsBuilder::from_opts(ops);
    let manager = MySqlConnectionManager::new(builder);
    R2D2Pool::builder()
        .max_size(5)
        .build(manager)
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    println!("Server is running on part: 3000");
    let pool: R2D2Pool<MySqlConnectionManager> = create_db_pool();

    let mut host_url = "".to_owned();
    let content = fs::read_to_string(".host");
    match content {
        Err(e) => {
            println!("{:?}", e);
            host_url = "127.0.0.1".to_owned();
        },
        Ok(s) => {
            host_url = s;
        }
    }
    let mut url = host_url.trim_end_matches(&['\r', '\n'][..]).to_string();
    url.push_str(":3000");
    println!("{}", url);
    return HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .service(get_tags)
            })
            .bind(url)?
            .run()
            .await;
}