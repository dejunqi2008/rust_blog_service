use crate::{
    model::tag::Tag,
    util::util::GenericError
};

use actix_web::{
    get,
    web::Json,
    web::Data
};
// use mysql::Pool;
use mysql::prelude::Queryable;
use r2d2::{Pool as R2D2Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;


#[get("/api/v2/tags")]
pub async fn get_tags(
    db :Data<R2D2Pool<MySqlConnectionManager>>
) -> Result<Json<Vec<Tag>>, GenericError> {

    let mut conn: PooledConnection<MySqlConnectionManager> = db.get().unwrap();
    let sql_str = "SELECT * FROM tags".to_string();

    let res = conn.query_map( sql_str, |(id, tagname, description)| Tag {
        id,
        tagname,
        description
    });

    return match res {
        Ok(item) => Ok(Json(item)),
        Err(e) => {
            // TODO: better handle different kind of error
            println!("{:?}", e.to_string());
            Err(GenericError::UnknowError)
        }
    };
}
