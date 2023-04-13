use crate::model::task::Task;
use crate::model::task::TaskState;
// use crate::repository::ddb::DDBRepository;
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use mysql::Pool;
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};
use derive_more::{Display};
//use std::fmt::{Display, Debug};
use r2d2::{Pool as R2D2Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;



#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[derive(Deserialize)]
pub struct TaskCompletionRequest {
    result_file: String
}

#[derive(Deserialize)]
pub struct SubmitTaskRequest {
    user_id: String,
    task_type: String,
    source_file: String
}


#[derive(Debug, Serialize)]
pub struct Tag {
    id: u32,
    tagname: String,
    description: Option<String>
}

#[derive(Debug, Display)]
pub enum GenericError {
    NotFound,
    UnknowError
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
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


#[get("/task/{task_global_id}")]
pub async fn get_task(
    task_identifier: Path<TaskIdentifier>
    // params: web::Path<UpdateParams>,
) -> Result<Json<Task>, TaskError> {
    println!("parameter: {:?}", task_identifier.into_inner().task_global_id);

    let tsk: Option<Task> = Some(Task {
        user_uuid: "ryw638q73".to_string(),
        task_uuid: "ashdkasd".to_string(),
        task_type: "Work".to_string(),
        state: TaskState::NotStarted,
        source_file: "Dummy file".to_string(),
        result_file: Some("Dummy result".to_string())
    });

    match tsk {
        Some(tsk) => Ok(Json(tsk)),
        None => Err(TaskError::TaskNotFound)
    }
}

// #[derive(Deserialize, Serialize)]
#[get("/api/v2/tags")]
pub async fn get_tags(
    db :Data<R2D2Pool<MySqlConnectionManager>>
) -> Result<Json<Vec<Tag>>, GenericError> {
    let mut conn: PooledConnection<MySqlConnectionManager> = db.get().unwrap();
    let s = "SELECT * FROM tags".to_string();

    let res = conn.query_map( s, |(id, tagname, description)| Tag {
        id,
        tagname,
        description
    }).expect("Query failed.");

    println!("{:?}", res);

    let mut tags: Vec<Tag> = vec![];
    let t1 = Tag {
        id: 1,
        tagname: "sport".to_string(),
        description: None
    };

    let t2 = Tag {
        id: 1,
        tagname: "economics".to_string(),
        description: None
    };

    tags.push(t1);
    tags.push(t2);
    Ok(Json(res))
}

