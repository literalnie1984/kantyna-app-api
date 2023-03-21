use actix_web::{get, post, web, Responder, HttpResponse};
use sea_orm::{Set, ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};

use bcrypt::{hash_with_salt, DEFAULT_COST, verify};
use nanoid::nanoid;

use entity::{user, order};
use entity::prelude::{User, Order};

use crate::appstate::AppState;

#[post("/login")]
async fn login(user: web::Json<user::Model>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;
    let user = user.into_inner();

    let user_query = User::find()
    .filter(user::Column::Username.eq(user.username))
    .one(conn)
    .await
    .unwrap();

    if user_query.is_none() {
        return HttpResponse::BadRequest().body("Account does not exist");
    }

    let result = verify(&user.password, &user_query.unwrap().password).unwrap();

    if result {
        HttpResponse::Ok().body("Good credentials")
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[post("/register")]
async fn register(user: web::Json<user::Model>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let user = user.into_inner();

    let salt = nanoid!(16);
    let salt_copy: [u8; 16] = salt.as_bytes().try_into().unwrap();
    let hashed_pass = hash_with_salt(user.password.as_bytes(), DEFAULT_COST, salt_copy).unwrap();

    user::ActiveModel {
        username: Set(user.username),
        password: Set(hashed_pass.to_string()),
        ..Default::default()
    }
    .save(conn)
    .await
    .unwrap();

    HttpResponse::Ok().body("Hello world")
}

#[get("/is_logged")]
async fn is_logged() -> impl Responder {
    "TODO"
}
