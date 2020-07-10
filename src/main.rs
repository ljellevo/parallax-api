

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_multipart_form_data;
extern crate formdata;


mod middleware;
use crate::middleware::MultipartError;
use crate::middleware::NewPayload;

type Result<T> = std::result::Result<T, MultipartError>;



use std::io::prelude::*;
use std::fs::File;
use serde::Serialize;
use rocket_contrib::json::Json;



#[get("/hello")]
fn hello() -> String {
	format!("Hello")
}

#[derive(Serialize)]
struct Task { 
	data: String
}

#[get("/api/upcoming_features")]
fn upcoming_features() -> Json<Task> {
	let mut file = File::open("markdown/upcoming_features.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/release_notes")]
fn release_notes() -> Json<Task> {
	let mut file = File::open("markdown/release_notes.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/how_it_works")]
fn how_it_works() -> Json<Task> {
	let mut file = File::open("markdown/how_it_works.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/about_us")]
fn about_us() -> Json<Task> {
	let mut file = File::open("markdown/about_us.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/privacy")]
fn privacy() -> Json<Task> {
	let mut file = File::open("markdown/privacy.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/terms_of_agreement")]
fn terms_of_agreement() -> Json<Task> {
	let mut file = File::open("markdown/terms_of_agreement.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}

#[get("/api/code_of_conduct")]
fn code_of_conduct() -> Json<Task> {
	let mut file = File::open("markdown/code_of_conduct.md").expect("Unable to open the file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Unable to read the file");
	Json(Task { data: contents})
}


#[derive(Serialize)]
struct Output {
	effect: String,
}

//https://blog.krruzic.xyz/rocket-multipart/
//https://docs.rs/rocket-multipart-form-data/0.9.3/rocket_multipart_form_data/
#[post("/api/image", data = "<multipart>")]
fn upload_image(multipart: Result<NewPayload>) -> String {
  
  match multipart {
    Ok(m) => {
      
      format!("Hello, {}", m.payload.effect)
    },
    Err(e) => format!("Error: {}", e.reason),
  }
}


fn main() {
	rocket::ignite().mount("/", routes![
		hello, 
		upcoming_features,
		release_notes,
		how_it_works,
		about_us,
		privacy,
		terms_of_agreement,
		code_of_conduct,
		upload_image
		]
	).launch();
}