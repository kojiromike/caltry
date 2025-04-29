#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use rocket_okapi::JsonSchema;
pub mod models;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct Message {
    id: usize,
    message: String,
}

/// Get a hello message
///
/// Returns a simple hello message.
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Get a message by ID
///
/// Returns a message object for the given ID.
#[openapi]
#[get("/message/<id>")]
fn get_message(id: usize) -> Json<Message> {
    Json(Message {
        id,
        message: format!("This is message number {}.", id),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", openapi_get_routes![index, get_message])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            })
        )
}
