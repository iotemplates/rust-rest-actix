use crate::models::CreatePerson;
use actix_web::web::Json;
use crate::data::{read_data, write_data};
use std::collections::HashMap;
use crate::models::Person;
use actix_web::web::Path;
use actix_web::{HttpResponse, Result};
use actix_web::http::{StatusCode};

pub async fn get_person(id: Path<String>) -> Result<HttpResponse> {
    let data:HashMap<String,Person> = HashMap::new();
    read_data(data).get(&id.to_string()).map(|p| Ok(HttpResponse::Ok().json(p))).unwrap_or(Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("Not found!")))
}
     
pub async fn get_persons() -> Result<HttpResponse> {
    let data:HashMap<String,Person> = HashMap::new();
    let persons: Vec<Person> = read_data(data).into_iter().map(|(_, v)| v).collect();
    Ok(HttpResponse::Ok().json(persons))
}

pub async fn create_person(p: Json<CreatePerson>) -> Result<HttpResponse> {
    let mut data: HashMap<String, Person> = read_data(HashMap::new());
    let next_id: i32 = data.clone().into_iter()
        .map(|(_, v)| v.id)
        .map(|s| s.parse::<i32>().expect("Failed to parse id."))
        .reduce(|a, b| {if a >= b {a} else {b}})
        .unwrap_or(0) + 1;

    let person = Person {id: next_id.to_string(), first_name: p.first_name.to_string(), last_name: p.last_name.to_string()};
    data.insert(next_id.to_string(), person.clone());
    write_data(data).expect("Failed to write persons.json");
    Ok(HttpResponse::Created().json(serde_json::to_string(&person).expect("Failed to serialize to json the specified person.")))
}

pub async fn delete_person(id: Path<String>) -> Result<HttpResponse> {
    let mut data: HashMap<String, Person> = read_data(HashMap::new());
    let removed: Option<Person> = data.remove(&id.to_string());
    if removed.is_some() {
        write_data(data).expect("Failed to save data in person.json.");
        Ok(HttpResponse::Accepted().json(serde_json::to_string(&removed.unwrap()).expect("Failed to serialize to json the specified person.")))
    } else {
        Ok(HttpResponse::NotFound().body(String::from("Not found!")))
    }
}
