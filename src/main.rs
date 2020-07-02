

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io::prelude::*;
use std::fs::File;

#[get("/hello")]
fn hello() -> String {
    format!("Hello")
}

#[get("/api/upcoming_features")]
fn upcoming_features() -> String {
  let mut file = File::open("markdown/upcoming_features.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/release_notes")]
fn release_notes() -> String {
  let mut file = File::open("markdown/release_notes.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/how_it_works")]
fn how_it_works() -> String {
  let mut file = File::open("markdown/how_it_works.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/about_us")]
fn about_us() -> String {
  let mut file = File::open("markdown/about_us.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/privacy")]
fn privacy() -> String {
  let mut file = File::open("markdown/privacy.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/terms_of_agreement")]
fn terms_of_agreement() -> String {
  let mut file = File::open("markdown/terms_of_agreement.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
}

#[get("/api/code_of_conduct")]
fn code_of_conduct() -> String {
  let mut file = File::open("markdown/code_of_conduct.md").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");
  format!("{}", contents)
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
      code_of_conduct
    ]
  ).launch();
}