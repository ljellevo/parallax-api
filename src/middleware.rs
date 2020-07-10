use rocket::data::{FromDataSimple, Outcome};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, RawField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MultipartError {
	pub reason: String,
}
impl MultipartError {
  fn new(reason: String) -> MultipartError {
    MultipartError { reason }
  }
}
impl std::fmt::Display for MultipartError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.reason)
  }
}

/// simple representation of a user
#[derive(Serialize, Deserialize)]
pub struct Payload {
	pub effect: String,
}

/// multipart form is loaded into this struct
/// this is what's passed through to the route we'll create later
pub struct NewPayload {
  /// the submitted image
  pub image: Vec<u8>,
  /// we'll deserialize the json into a User
  pub payload: Payload,
}


impl FromDataSimple for NewPayload {
	type Error = MultipartError;

	fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
    let image_bytes;
    let post_obj;
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields.push(
      MultipartFormDataField::raw("image")
        .size_limit(8 * 1024 * 1024) // 8 MB
        .content_type_by_string(Some(mime::IMAGE_STAR))
        .unwrap(),
    );
  
    options.allowed_fields.push(MultipartFormDataField::text("data").content_type(Some(mime::STAR_STAR)));

    let content_type = match request.content_type() {
      Some(content_type) => content_type,
      None => {
        return Failure((
          Status::BadRequest,
          MultipartError::new(format!("Incorrect contentType, should be 'multipart/form-data")),
        ))
      }
    };

    let multipart_form = match MultipartFormData::parse(&content_type, data, options) {
      Ok(m) => m,
      Err(e) => {
        return Failure((Status::BadRequest, MultipartError::new(format!("{:?}", e))))
      }
    };

    let post_json_part = match multipart_form.texts.get("data") {
      Some(post_json_part) => post_json_part,
      _ => {
          return Failure((
            Status::BadRequest,
            MultipartError::new(format!("Missing field 'data'")),
        ))
      }
    };

    let image_part: &Vec<RawField> = match multipart_form.raw.get("image") {
      Some(image_part) => image_part,
      _ => {
        return Failure((
          Status::BadRequest,
          MultipartError::new(format!("Missing field 'image'")),
        ))
      }
    };

    match post_json_part.len() {
      1 => {
        let json_string = &post_json_part[0].text.replace('\'', "\"");
        post_obj = match serde_json::from_str::<Payload>(json_string) {
          Ok(insert) => insert,
          Err(e) => {
            return Failure((
              Status::BadRequest,
              MultipartError::new(format!("{:?}", e)),
            ))
          }
        };
      }
      _ => {
        return Failure((
          Status::BadRequest,
          MultipartError::new(format!("Extra text fields supplied")),
        ))
      } 
    };
      
    match image_part.len() {
      
      1 => {
        image_bytes = image_part[0].raw.clone();
        print!("{:?}", image_bytes);
      }
      _ => {
        return Failure((
          Status::BadRequest,
          MultipartError::new(format!("Extra image fields supplied")),
        ))
      }
    };
    

    Success(NewPayload {
      payload: post_obj,
      image: image_bytes,
    })
	}
}