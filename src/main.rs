mod api;
mod model;
mod util;

use mysql::{prelude::*, Opts, OptsBuilder};

use actix_web::{web, App, HttpServer};
use r2d2::{Pool as R2D2Pool};
use r2d2_mysql::MySqlConnectionManager;
use crate::api::tag::get_tags;

fn create_db_pool() -> R2D2Pool<MySqlConnectionManager> {
    let url = "mysql://root:dejunqilocal@localhost:3306/myblog";
    let ops = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(ops);
    let manager = MySqlConnectionManager::new(builder);
    R2D2Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server is running on part: 3000");
    // let pool: R2D2Pool<MySqlConnectionManager> = create_db_pool();

    return HttpServer::new(move || {
            App::new()
            // .app_data(web::Data::new(pool.clone()))
                .service(get_tags)
            })
            .bind("34.220.237.129:3000")?
            .run()
            .await;
}
