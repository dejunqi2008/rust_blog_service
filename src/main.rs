mod api;
mod model;


// use api::task:: {get_task};

use mysql::{prelude::*, Opts, OptsBuilder};
use chrono::prelude::*; // 用来处理日期
use actix_web::{web, App, HttpServer};
use mysql::Pool;
use r2d2::{Pool as R2D2Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;

// type DbPool = r2d2::Pool<r2d2::PooledConnection<MySql>>

use api::task::{
    get_task,
    submit_task,
};




/*
fn test_mysql() {
    // mysql://[db_user]:[db_password]@[db_host]:[db_port]/[db_name]
    let url = "mysql://root:dejunqilocal@localhost:3306/myblog";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap();// 获取链接

    let s = "SELECT * FROM tags".to_string();

    let res = conn.query_map( s, |(id, tagname, description)| Tag {
        id,
        tagname,
        description
    }).expect("Query failed.");

    for i in res {
        println!("{}, {}, {}", i.id, i.tagname, i.description.unwrap_or("NONE".to_string()));
    }

}
*/


struct AppState {
    db_pool: R2D2Pool<MySqlConnectionManager>
}

// async fn establish_connection() -> Pool {
//     let database_url = "mysql://root:dejunqilocal@localhost:3306/myblog";
//     let pool = Pool::new(database_url).unwrap();
//     pool
// }

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

    println!("Hello, world!");
    let pool: R2D2Pool<MySqlConnectionManager> = create_db_pool();

    // Pool<MySqlConnectionManager>
    return HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/task/{task_global_id}", web::get().to(get_task))
            })
            .bind("127.0.0.1:3000")?
            .run()
            .await;
}

/*
mysql://b127a39af6a3d6:25ed336e@us-cdbr-east-06.cleardb.net/heroku_7062e7dfec0556b?reconnect=true
*/