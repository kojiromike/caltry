pub mod models;

use butane::Error;
use butane::FromSql;
use butane::db::{Connection, ConnectionSpec};
use butane::{find, query};
use butane::{prelude::*, AutoPk, SqlVal};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{ Alarm, Attendee, Calendar, Event, Journal, Todo, };


pub fn connection_string() -> ConnectionSpec {
    /* let mut connection = butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap();
    let migrations = butane_migrations::get_migrations().unwrap();
    migrations.migrate(&mut connection).unwrap();
    connection
    */
    ConnectionSpec::load(".butane/connection.json").unwrap()

    // butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap()
}

pub fn create_calendar(
    conn: &Connection,
    prod_id: impl Into<String>,
    version: impl Into<String>,
) -> Calendar {
    let mut calendar = Calendar::new(prod_id, version);
    calendar.save(conn).unwrap();
    calendar
}

pub fn create_event(
    conn: &Connection,
    calendar: &Calendar,
    summary: Option<String>,
    dtstart: DateTime<Utc>,
    dtend: Option<DateTime<Utc>>,
    location: Option<String>,
    description: Option<String>,
) -> Event {
    let uid = Uuid::new_v4().to_string();
    let mut event = Event::new(calendar, uid, dtstart);

    event.summary = summary;
    event.dtend = dtend;
    event.location = location;
    event.description = description;

    event.save(conn).unwrap();
    event
}

// Create an event with full customization using a closure
pub fn create_event_with<F>(
    conn: &Connection,
    calendar: &Calendar,
    dtstart: DateTime<Utc>,
    customize: F,
) -> Event
where
    F: FnOnce(&mut Event),
{
    let uid = Uuid::new_v4().to_string();
    let mut event = Event::new(calendar, uid, dtstart);

    // Apply customizations
    customize(&mut event);

    event.save(conn).unwrap();
    event
}

// Simplified version that uses builder pattern
pub fn create_event_builder(
    calendar: &Calendar,
    dtstart: DateTime<Utc>,
) -> Event {
    let uid = Uuid::new_v4().to_string();
    let event = Event::new(calendar, uid, dtstart);

    // Return the event without saving, so builders can be used
    // The caller must call .save(conn) at the end
    event
}

pub fn create_todo(
    conn: &Connection,
    calendar: &Calendar,
    summary: Option<String>,
    description: Option<String>,
    due: Option<DateTime<Utc>>,
    priority: Option<i32>,
) -> Todo {
    let uid = Uuid::new_v4().to_string();
    let mut todo = Todo::new(calendar, uid);

    todo.summary = summary;
    todo.description = description;
    todo.due = due;
    todo.priority = priority;

    todo.save(conn).unwrap();
    todo
}

// Create a todo with full customization using a closure
pub fn create_todo_with<F>(
    conn: &Connection,
    calendar: &Calendar,
    customize: F,
) -> Todo
where
    F: FnOnce(&mut Todo),
{
    let uid = Uuid::new_v4().to_string();
    let mut todo = Todo::new(calendar, uid);

    // Apply customizations
    customize(&mut todo);

    todo.save(conn).unwrap();
    todo
}

// Create a todo using the comprehensive constructor
pub fn create_todo_with_details(
    conn: &Connection,
    calendar: &Calendar,
    dtstart: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    completed: Option<DateTime<Utc>>,
    duration: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    location: Option<String>,
    status: Option<String>,
    priority: Option<i32>,
    percent_complete: Option<i32>,
    organizer: Option<String>,
    class: Option<String>,
    categories: Option<String>,
    rrule: Option<String>,
) -> Todo {
    let uid = Uuid::new_v4().to_string();
    let mut todo = Todo::new_with_details(
        calendar,
        uid,
        dtstart,
        due,
        completed,
        duration,
        summary,
        description,
        location,
        status,
        priority,
        percent_complete,
        organizer,
        class,
        categories,
        rrule,
    );

    todo.save(conn).unwrap();
    todo
}

pub fn create_journal(
    conn: &Connection,
    calendar: &Calendar,
    summary: Option<String>
) -> Journal {
    let uid = Uuid::new_v4().to_string();
    let mut journal = Journal::new(calendar, uid);
    journal.summary = summary;
    journal.save(conn).unwrap();
    journal
}

pub fn create_alarm(
    conn: &Connection,
    action: impl Into<String>,
    trigger: impl Into<String>
) -> Alarm {
    let mut alarm = Alarm::new(action.into(), trigger.into());
    alarm.save(conn).unwrap();
    alarm
}

pub fn create_attendee(
    conn: &Connection,
    cal_address: impl Into<String>
) -> Attendee {
    let mut attendee = Attendee::new(cal_address.into());
    attendee.save(conn).unwrap();
    attendee
}

pub fn existing_calendar(conn: &Connection) -> Option<Calendar> {
    Calendar::query().load_first(conn).unwrap()
}

pub fn find_events_by_summary(conn: &Connection, summary_text: &str) -> Vec<Event> {
    let search_text = format!("%{}%", summary_text);
    query!(Event, summary.like({search_text}))
        .load(conn)
        .unwrap()
}

pub fn find_todos_by_summary(conn: &Connection, summary_text: &str) -> Vec<Todo> {
    let search_text = format!("%{}%", summary_text);
    query!(Todo, summary.like({search_text}))
        .load(conn)
        .unwrap()
}

pub fn get_calendars(conn: &Connection) -> Vec<Calendar> {
    Calendar::query().load(conn).unwrap()
}

pub fn get_calendar_by_id(conn: &Connection, id_value: i64) -> Result<Calendar, Error> {
    // This is a workaround since AutoPk doesn't have a direct constructor from i64
    let pk_sqlval = SqlVal::BigInt(id_value);
    let pk = AutoPk::from_sql(pk_sqlval).unwrap();
    // let pk = AutoPk::with_value(id_value);
    find!(Calendar, id == { pk }, conn)
}

pub fn get_events(conn: &Connection) -> Vec<Event> {
    Event::query().load(conn).unwrap()
}

pub fn get_event_by_id(conn: &Connection, id_value: i64) -> Result<Event, Error> {
    // This is a workaround since AutoPk doesn't have a direct constructor from i64
    let pk_sqlval = SqlVal::BigInt(id_value);
    let pk = AutoPk::from_sql(pk_sqlval).unwrap();
    // let pk = AutoPk::with_value(id_value);
    find!(Event, id == { pk }, conn)
}

pub fn get_todos(conn: &Connection) -> Vec<Todo> {
    Todo::query().load(conn).unwrap()
}
