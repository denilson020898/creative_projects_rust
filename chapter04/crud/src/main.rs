mod db_access;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::sync::Mutex;

use db_access::Person;

struct AppState {
    db: db_access::DbConnection,
}

#[derive(Deserialize)]
pub struct ToDelete {
    id_list: Option<String>,
}

#[derive(Deserialize)]
pub struct ToInsert {
    name: Option<String>,
}

#[derive(Deserialize)]
pub struct ToUpdate {
    id: Option<u32>,
    name: Option<String>,
}

#[derive(Deserialize)]
pub struct Filter {
    partial_name: Option<String>,
}

lazy_static! {
    pub static ref TERA: tera::Tera = tera::Tera::new("templates/**/*").unwrap();
}

fn get_main() -> impl Responder {
    let context = tera::Context::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("main.html", &context).unwrap())
}

fn get_page_persons(
    query: web::Query<Filter>,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let partial_name = &query.partial_name.clone().unwrap_or_else(|| "".to_string());
    let db_conn = &state.lock().unwrap().db;
    let person_list = db_conn.get_persons_by_partial_name(&partial_name);
    let mut context = tera::Context::new();
    context.insert("id_error", &"");
    context.insert("partial_name", &partial_name);
    context.insert("persons", &person_list.collect::<Vec<_>>());

    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn get_favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(include_bytes!("favicon.ico") as &[u8])
}

fn delete_persons(
    query: web::Query<ToDelete>,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let db_conn = &mut state.lock().unwrap().db;
    let mut deleted_count = 0;
    query
        .id_list
        .clone()
        .unwrap_or_else(|| "".to_string())
        .split_terminator(",")
        .for_each(|id| {
            deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                1
            } else {
                0
            };
        });
    deleted_count.to_string()
}

fn get_page_new_person() -> impl Responder {
    let mut context = tera::Context::default();
    context.insert("person_id", &"");
    context.insert("person_name", &"");
    context.insert("inserting", &true);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("one_person.html", &context).unwrap())
}

fn get_page_edit_person(
    state: web::Data<Mutex<AppState>>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let id = &path.0;
    let db_conn = &state.lock().unwrap().db;
    let mut context = tera::Context::new();
    if let Ok(id_n) = id.parse::<u32>() {
        if let Some(person) = db_conn.get_person_by_id(id_n) {
            context.insert("person_id", &id);
            context.insert("person_name", &person.name);
            context.insert("inserting", &false);
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(TERA.render("one_person.html", &context).unwrap());
        }
    }
    context.insert("id_error", &"Person id not found");
    context.insert("partial_name", &"");
    let person_list = db_conn.get_persons_by_partial_name(&"");
    context.insert("persons", &person_list.collect::<Vec<_>>());

    HttpResponse::Ok()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn insert_person(state: web::Data<Mutex<AppState>>, query: web::Query<ToInsert>) -> impl Responder {
    let db_conn = &mut state.lock().unwrap().db;
    let mut inserted_count = 0;
    if let Some(name) = &query.name.clone() {
        inserted_count += db_conn.insert_person(Person {
            id: 0,
            name: name.clone(),
        });
    }
    inserted_count.to_string()
}

fn update_person(state: web::Data<Mutex<AppState>>, query: web::Query<ToUpdate>) -> impl Responder {
    let db_conn = &mut state.lock().unwrap().db;
    let mut updated_count = 0;
    let id = query.id.unwrap_or(0);
    let name = query.name.clone().unwrap_or_else(|| "".to_string()).clone();
    updated_count += if db_conn.update_person(Person { id, name }) {
        1
    } else {
        0
    };
    updated_count.to_string()
}

fn invalid_resource(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let db_conn = &state.lock().unwrap().db;
    let mut context = tera::Context::new();
    context.insert("id_error", &"Invalid request.");
    context.insert("partial_name", &"");
    let person_list = db_conn.get_persons_by_partial_name(&"");
    context.insert("persons", &person_list.collect::<Vec<_>>());

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(TERA.render("persons.html", &context).unwrap())
}

fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("listening at address: {}", server_address);

    let db_conn = web::Data::new(Mutex::new(AppState {
        db: db_access::DbConnection::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .register_data(db_conn.clone())
            .service(web::resource("/").route(web::get().to(get_main)))
            .service(web::resource("/page/persons").route(web::get().to(get_page_persons)))
            .service(web::resource("/persons").route(web::delete().to(delete_persons)))
            .service(web::resource("/page/new_person").route(web::get().to(get_page_new_person)))
            .service(
                web::resource("/page/edit_person/{id}").route(web::get().to(get_page_edit_person)),
            )
            .service(
                web::resource("/one_person")
                    .route(web::post().to(insert_person))
                    .route(web::put().to(update_person)),
            )
            .service(web::resource("/favicon.ico").route(web::get().to(get_favicon)))
            .default_service(web::get().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
}
