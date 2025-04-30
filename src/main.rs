#[macro_use] extern crate rocket;
use butane::db::{Connection, ConnectionManager};
use caltry::{ get_calendar_by_id, find_events_by_summary, get_event_by_id, find_todos_by_summary, };
use chrono::{DateTime, Utc};
use r2d2;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_okapi::JsonSchema;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use std::ops::Deref;

// Type alias for the connection pool
type Pool = r2d2::Pool<ConnectionManager>;

// Connection wrapper that retrieves connections from the pool
struct DbConn(r2d2::PooledConnection<ConnectionManager>);

// Implement Deref to get access to the connection methods
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = request.rocket().state::<Pool>().unwrap();
        match pool.get() {
            Ok(conn) => rocket::request::Outcome::Success(DbConn(conn)),
            Err(_) => rocket::request::Outcome::Error((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for DbConn {
    fn from_request_input(
        _generator: &mut rocket_okapi::r#gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Database connections don't need to be documented in the OpenAPI schema
        Ok(RequestHeaderInput::None)
    }
}
// DTO Models
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CalendarRequest {
    prod_id: String,
    version: String,
    calscale: Option<String>,
    method: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CalendarResponse {
    id: i64,
    prod_id: String,
    version: String,
    calscale: Option<String>,
    method: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct EventRequest {
    calendar_id: i64,
    summary: Option<String>,
    dtstart: DateTime<Utc>,
    dtend: Option<DateTime<Utc>>,
    description: Option<String>,
    location: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct EventResponse {
    id: i64,
    uid: String,
    summary: Option<String>,
    dtstart: DateTime<Utc>,
    dtend: Option<DateTime<Utc>>,
    description: Option<String>,
    location: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct TodoRequest {
    calendar_id: i64,
    summary: Option<String>,
    description: Option<String>,
    due: Option<DateTime<Utc>>,
    priority: Option<i32>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct TodoResponse {
    id: i64,
    uid: String,
    summary: Option<String>,
    description: Option<String>,
    due: Option<DateTime<Utc>>,
    completed: Option<DateTime<Utc>>,
}

/// Get a hello message
///
/// Returns a simple hello message.
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Get all calendars
///
/// Returns a list of all calendars
#[openapi]
#[get("/calendars")]
fn get_calendars(conn: DbConn) -> Json<Vec<CalendarResponse>> {
    let calendars = caltry::get_calendars(&conn);

    let responses: Vec<CalendarResponse> = calendars.into_iter().map(|cal| {
        CalendarResponse {
            id: cal.id.unwrap(),
            prod_id: cal.prod_id,
            version: cal.version,
            calscale: cal.calscale,
            method: cal.method,
        }
    }).collect();

    Json(responses)
}

/// Get a calendar by ID
///
/// Returns a calendar for the given ID
#[openapi]
#[get("/calendars/<id>")]
fn get_calendar(conn: DbConn, id: i64) -> Option<Json<CalendarResponse>> {
    let calendar = get_calendar_by_id(&conn, id).ok()?;

    Some(Json(CalendarResponse {
        id: calendar.id.unwrap(),
        prod_id: calendar.prod_id,
        version: calendar.version,
        calscale: calendar.calscale,
        method: calendar.method,
    }))
}

/// Create a new calendar
///
/// Creates and returns a new calendar
#[openapi]
#[post("/calendars", format = "json", data = "<calendar>")]
fn create_calendar(conn: DbConn, calendar: Json<CalendarRequest>) -> Json<CalendarResponse> {
    let cal = caltry::create_calendar(
        &conn,
        calendar.prod_id.clone(),
        calendar.version.clone()
    );

    Json(CalendarResponse {
        id: cal.id.unwrap(),
        prod_id: cal.prod_id,
        version: cal.version,
        calscale: cal.calscale,
        method: cal.method,
    })
}

/// Get all events
///
/// Returns all events or events filtered by summary
#[openapi]
#[get("/events?<summary>")]
fn get_events(conn: DbConn, summary: Option<&str>) -> Json<Vec<EventResponse>> {
    match summary  {
        Some(text) => {
            let events = crate::find_events_by_summary(&conn, text);
            if events.is_empty() {
                return Json(vec![]);
            };
            let responses = events.into_iter().map(|event| {
                EventResponse {
                    id: event.id.unwrap(),
                    uid: event.uid,
                    summary: event.summary,
                    dtstart: event.dtstart,
                    dtend: event.dtend,
                    description: event.description,
                    location: event.location,
                }
            }).collect();
            Json(responses)
        },
        None => {
            let events = caltry::get_events(&conn);
            if events.is_empty() {
                return Json(vec![]);
            };
            let responses = events.into_iter().map(|event| {
                EventResponse {
                    id: event.id.unwrap(),
                    uid: event.uid,
                    summary: event.summary,
                    dtstart: event.dtstart,
                    dtend: event.dtend,
                    description: event.description,
                    location: event.location,
                }
            }).collect();
            Json(responses)
        }
    }


}

/// Get an event by ID
///
/// Returns an event for the given ID
#[openapi]
#[get("/events/<id>")]
fn get_event(conn: DbConn, id: i64) -> Option<Json<EventResponse>> {
    let event = crate::get_event_by_id(&conn, id).ok()?;

    Some(Json(EventResponse {
        id: event.id.unwrap(),
        uid: event.uid,
        summary: event.summary,
        dtstart: event.dtstart,
        dtend: event.dtend,
        description: event.description,
        location: event.location,
    }))
}

/// Create a new event
///
/// Creates and returns a new event
#[openapi]
#[post("/events", format = "json", data = "<event>")]
fn create_event(conn: DbConn, event: Json<EventRequest>) -> Json<EventResponse> {
    let calendar_id = event.calendar_id;
    let calendar = get_calendar_by_id(&conn, calendar_id).unwrap();
    let new_event = caltry::create_event(
        &conn,
        &calendar,
        event.summary.clone(),
        event.dtstart,
        event.dtend,
        event.location.clone(),
        event.description.clone(),
    );

    Json(EventResponse {
        id: new_event.id.unwrap(),
        uid: new_event.uid,
        summary: new_event.summary,
        dtstart: new_event.dtstart,
        dtend: new_event.dtend,
        description: new_event.description,
        location: new_event.location,
    })
}

/// Get all todos
///
/// Returns all todos or todos filtered by summary
#[openapi]
#[get("/todos?<summary>")]
fn get_todos(conn: DbConn, summary: Option<&str>) -> Json<Vec<TodoResponse>> {
    match summary {
        Some(text) => {
            let todos = crate::find_todos_by_summary(&conn, text);
            if todos.is_empty() {
                return Json(vec![]);
            };
            let responses = todos.into_iter().map(|todo| {
                TodoResponse {
                    id: todo.id.unwrap(),
                    uid: todo.uid,
                    summary: todo.summary,
                    description: todo.description,
                    due: todo.due,
                    completed: todo.completed,
                }
            }).collect();
            Json(responses)
        },
        None => {
            let todos = caltry::get_todos(&conn);
            if todos.is_empty() {
                return Json(vec![]);
            };
            let responses = todos.into_iter().map(|todo| {
                TodoResponse {
                    id: todo.id.unwrap(),
                    uid: todo.uid,
                    summary: todo.summary,
                    description: todo.description,
                    due: todo.due,
                    completed: todo.completed,
                }
            }).collect();
            Json(responses)
        }
    }
}

/// Create a new todo
///
/// Creates and returns a new todo
#[openapi]
#[post("/todos", format = "json", data = "<todo>")]
fn create_todo(conn: DbConn, todo: Json<TodoRequest>) -> Json<TodoResponse> {
    let calendar = get_calendar_by_id(&conn, todo.calendar_id).unwrap();
    let new_todo = caltry::create_todo(
        &conn,
        &calendar,
        todo.summary.clone(),
        todo.description.clone(),
        todo.due.clone(),
        todo.priority.clone(),
    );

    Json(TodoResponse {
        id: new_todo.id.unwrap(),
        uid: new_todo.uid,
        summary: new_todo.summary,
        description: new_todo.description,
        due: new_todo.due,
        completed: new_todo.completed,
    })
}

#[launch]
fn rocket() -> _ {
    // Create a connection manager
    let manager = ConnectionManager::new(caltry::connection_string());

    // Create a connection pool
    let pool = r2d2::Pool::builder()
        .max_size(15) // Adjust pool size as needed
        .build(manager)
        .expect("Failed to create database pool");

    rocket::build()
        .mount("/api", openapi_get_routes![
            index,
            get_calendars, get_calendar, create_calendar,
            get_events, get_event, create_event,
            get_todos, create_todo
        ])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            })
        )
        .manage(pool) // Store the pool instead of single connection
}
