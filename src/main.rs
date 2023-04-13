mod api;
mod model;

use mysql::{prelude::*, Opts, OptsBuilder};
use chrono::prelude::*; // 用来处理日期
use actix_web::{web, App, HttpServer};
// use mysql::Pool;
use r2d2::{Pool as R2D2Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;

// type DbPool = r2d2::Pool<r2d2::PooledConnection<MySql>>

use api::task::{
    get_task,
    get_tags
};




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
    let pool: R2D2Pool<MySqlConnectionManager> = create_db_pool();

    return HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .service(get_task)
                .service(get_tags)
            })
            .bind("127.0.0.1:3000")?
            .run()
            .await;
}
