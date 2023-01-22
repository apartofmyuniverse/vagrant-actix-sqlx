use actix_web::{post, get, Responder, HttpResponse};
use actix_web::web::{Data, Path, Json};

use sqlx::{self, FromRow};

use serde::{Serialize, Deserialize};

use crate::AppState;

use log::{info,warn};

#[derive(Serialize, FromRow)]
struct User {
    id : i64,
    first_name : String,
    last_name : String,
}

#[derive(Serialize,FromRow)]
struct Article {
    id: i64,
    title: String,
    content: String,
    created_by : i64,
}

/********************************************
 * Struct used to carry Article insert data *
 ********************************************/
#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title : String,
    pub content : String,
}

/********************************************************************************************
 * NAME: create_user_article                                                                *
 ********************************************************************************************
 * PARAMS:                                                                                  *
 *          - state => Contains reference to DB                                             *
 *          - path  => Contains user reference id                                           *
 *          - body  => Contains insert data (marshalled into a CreateArticleBody struct)    *
 ********************************************************************************************
 * RETURN                                                                                   *
 *          - Article Struct                                                                *
 *              - id                                                                        *
 *              - title                                                                     *
 *              - content                                                                   *
 *              - created_by                                                                *
 ********************************************************************************************/
 
#[post("/users/{id}/articles")]
pub async fn create_user_article(state : Data<AppState>, path:Path<i32>, body : Json<CreateArticleBody>) -> impl Responder {
    let id : i32 = path.into_inner();
    info!("Title: {} Content : {}", body.title.to_string(), body.content.to_string());
    match sqlx::query_as::<_, Article>("INSERT INTO articles (title, content, created_by) VALUES ($1,$2,$3) RETURNING id, title, content, created_by")
    .bind(body.title.to_string())
    .bind(body.content.to_string())
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(err) => {warn!("{}",err); HttpResponse::InternalServerError().json("Failed to create user article")},
    }
    //format!("POST /users/{id}/articles")
}

#[get("/users")]
pub async fn fetch_users(state : Data<AppState>) -> impl Responder {
    //format!("GET /users/")
    match sqlx::query_as::<_, User>("SELECT id, first_name, last_name FROM users")
        .fetch_all(&state.db)
        .await
        {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(err) => {warn!("{}",err); HttpResponse::NotFound().json("No users found")}, 
        }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state : Data<AppState>, path:Path<i32>) -> impl Responder {
    let id : i32 = path.into_inner();
    match sqlx::query_as::<_, Article>("SELECT id, title, content, created_by FROM articles where created_by = $1")
        .bind(&id)
        .fetch_all(&state.db)
        .await
        {
            Ok(articles) => HttpResponse::Ok().json(articles),
            Err(err) => {warn!("{}", err); HttpResponse::NotFound().json(format!("No articles found for user : {}",id))},
        }

}