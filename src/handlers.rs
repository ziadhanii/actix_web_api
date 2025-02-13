use crate::db::DbPool;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");
    match users.load::<User>(&mut conn) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let new_user = user.into_inner();

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match users.find(*user_id).first::<User>(&mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match diesel::update(users.find(*user_id))
        .set((name.eq(&user.name), email.eq(&user.email)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match diesel::delete(users.find(*user_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
