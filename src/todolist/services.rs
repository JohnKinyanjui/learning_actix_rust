use actix_web::{get, post, delete, web, Responder, HttpResponse};
use serde::Serialize;
use crate::{AppState, TodoListEntry};

use super::models::{CreateEntryData,UpdateEntryData};


#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/add")]
async fn create_entry(data :web::Data<AppState>,param_obj :web::Json<CreateEntryData>) -> impl Responder{
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id = 0;

    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id{
            max_id = todolist_entries[i].id;
        }
    }

    todolist_entries.push(TodoListEntry { id: max_id+1, title: param_obj.title.clone(), date: param_obj.data,});


    HttpResponse::Ok().json(todolist_entries.to_vec())

}

#[delete("/todolist/entry/{id}")]
async fn delete_entry(data :web::Data<AppState>, path :web::Path<i32>) -> impl Responder{
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let current_length = todolist_entries.len();

    let id = path.into_inner();
    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

    if current_length == todolist_entries.len(){

        #[derive(Serialize)]
        struct Response{
            error: String
        }
        return HttpResponse::Ok().json(Response{error: "this item does not exists".to_string()});
    }

    
    HttpResponse::Ok().json(todolist_entries.to_vec())

}


pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_entries)
        .service(create_entry)
        .service(delete_entry);
}