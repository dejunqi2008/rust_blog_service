mod api;

// use api::task:: {get_task};
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*; // 用来处理日期


struct Tag {
    id: u32,
    tagname: String,
    description: Option<String>
}

fn main() {
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

    println!("Hello, world!");
}

/*
mysql://b127a39af6a3d6:25ed336e@us-cdbr-east-06.cleardb.net/heroku_7062e7dfec0556b?reconnect=true
*/