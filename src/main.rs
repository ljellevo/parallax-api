#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_multipart_form_data;

mod middleware;
use rocket::Data;
use crate::middleware::ApiKey;
use std::io::prelude::*;
use std::fs::File;
use serde::Serialize;
use rocket_contrib::json::Json;

type Result<T> = std::result::Result<T, ApiKey>;

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


#[post("/api/<effect>", data="<data>")]
fn upload_image(effect: String, data: Data, key: ApiKey) {
  println!("Effect is {}", effect);
  let mut content = data.open();
  let buffer = &mut Vec::new();
  match content.read_to_end(buffer) {
    Ok(_) =>  {
      println!("Image was recieved");
    },
    Err(_) => {
      println!("No image was recieved");
    }
  }
}


fn main() {
	rocket::ignite().mount("/", routes![
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