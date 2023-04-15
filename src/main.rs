mod util;
use actix_web::{
    web,
    web::Json,
    get,
    App,
    HttpServer,
    ResponseError,
    HttpResponse,
    http::{
        header::ContentType,
        StatusCode
    }
};
use std::fs;
use serde::Serialize;
use derive_more::Display;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub id: u32,
    pub tagname: String,
    pub description: Option<String>
}


#[derive(Debug, Display)]
pub enum GenericError {
    NotFound,
    UnknowError
}

impl ResponseError for GenericError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            GenericError::NotFound => StatusCode::NOT_FOUND,
            GenericError::UnknowError => StatusCode::BAD_REQUEST
        }
    }
}


// Use hard coded dummy data for now
#[get("/api/v2/tags")]
pub async fn get_tags() -> Result<Json<Vec<Tag>>, GenericError> {
    let t1 = Tag {
        id: 1,
        tagname: "sport".to_string(),
        description: None
    };
    let t2 = Tag {
        id: 2,
        tagname: "economics".to_string(),
        description: Some("Topics about economic".to_string())
    };

    let mut res: Vec<Tag> = vec![];
    res.push(t1);
    res.push(t2);
    return Ok(Json(res));
}

// use dummy data for now, replace it with db pool once get MySql setup
#[derive(Debug, Default)]
struct ActixData {
    counter: usize,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    println!("Environment var: {:?}", std::env::var("DUMMY_VAR"));
    env_logger::init();
    println!("Server is running on part: 3000");
    let data = web::Data::new(ActixData {
        counter: 0
    });

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
            .app_data(data.clone())
                .service(get_tags)
            })
            .bind(url)?
            .run()
            .await;
}
