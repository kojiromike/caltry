use butane::{model, AutoPk, ForeignKey, Many};
use chrono::{DateTime, Utc};

// Calendar - represents VCALENDAR
#[model]
#[derive(Debug, Default)]
pub struct Calendar {
    pub id: AutoPk<i64>,
    pub prod_id: String,      // PRODID property (required)
    pub version: String,      // VERSION property (required)
    pub calscale: Option<String>, // CALSCALE property (optional)
    pub method: Option<String>,   // METHOD property (optional)
}

impl Calendar {
    pub fn new(prod_id: impl Into<String>, version: impl Into<String>) -> Self {
        Calendar {
            id: AutoPk::uninitialized(),
            prod_id: prod_id.into(),
            version: version.into(),
            calscale: Some("GREGORIAN".to_string()),
            method: None,
        }
    }
}

// Event - represents VEVENT
#[model]
pub struct Event {
    pub id: AutoPk<i64>,
    pub calendar: ForeignKey<Calendar>,
    pub uid: String,          // UID property (required)
    pub dtstamp: DateTime<Utc>, // DTSTAMP property (required)
    pub dtstart: DateTime<Utc>, // DTSTART property
    pub dtend: Option<DateTime<Utc>>, // DTEND property
    pub duration: Option<String>, // DURATION property (using ISO 8601 format)
    pub summary: Option<String>, // SUMMARY property
    pub description: Option<String>, // DESCRIPTION property
    pub location: Option<String>, // LOCATION property
    pub status: Option<String>, // STATUS property
    pub organizer: Option<String>, // ORGANIZER property
    pub url: Option<String>,  // URL property
    pub transp: Option<String>, // TRANSP property
    pub class: Option<String>, // CLASS property
    pub categories: Option<String>, // CATEGORIES property (comma-separated)
    pub created: Option<DateTime<Utc>>, // CREATED property
    pub last_modified: Option<DateTime<Utc>>, // LAST-MODIFIED property
    pub sequence: i32,        // SEQUENCE property
    pub rrule: Option<String>, // RRULE property
    pub alarms: Many<Alarm>,  // VALARM components
    pub attendees: Many<Attendee>, // ATTENDEE properties
}

impl Event {
    pub fn new(calendar: &Calendar, uid: String, dtstart: DateTime<Utc>) -> Self {
        Event {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            uid,
            dtstamp: Utc::now(),
            dtstart,
            dtend: None,
            duration: None,
            summary: None,
            description: None,
            location: None,
            status: None,
            organizer: None,
            url: None,
            transp: None,
            class: None,
            categories: None,
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            sequence: 0,
            rrule: None,
            alarms: Many::default(),
            attendees: Many::default(),
        }
    }

    // Add a more comprehensive constructor with all fields
    pub fn new_with_details(
        calendar: &Calendar,
        uid: String,
        dtstart: DateTime<Utc>,
        dtend: Option<DateTime<Utc>>,
        duration: Option<String>,
        summary: Option<String>,
        description: Option<String>,
        location: Option<String>,
        status: Option<String>,
        organizer: Option<String>,
        url: Option<String>,
        transp: Option<String>,
        class: Option<String>,
        categories: Option<String>,
        rrule: Option<String>,
    ) -> Self {
        Event {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            uid,
            dtstamp: Utc::now(),
            dtstart,
            dtend,
            duration,
            summary,
            description,
            location,
            status,
            organizer,
            url,
            transp,
            class,
            categories,
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            sequence: 0,
            rrule,
            alarms: Many::default(),
            attendees: Many::default(),
        }
    }

    // Builder pattern methods for fluent API
    pub fn with_dtend(mut self, dtend: DateTime<Utc>) -> Self {
        self.dtend = Some(dtend);
        self
    }

    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_organizer(mut self, organizer: impl Into<String>) -> Self {
        self.organizer = Some(organizer.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_transp(mut self, transp: impl Into<String>) -> Self {
        self.transp = Some(transp.into());
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn with_categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    pub fn with_rrule(mut self, rrule: impl Into<String>) -> Self {
        self.rrule = Some(rrule.into());
        self
    }

    pub fn with_sequence(mut self, sequence: i32) -> Self {
        self.sequence = sequence;
        self
    }

    // Methods to add alarms and attendees
    pub fn add_alarm(mut self, alarm: Alarm) -> Self {
        self.alarms.add(&alarm).expect("Failed to add alarm");
        self
    }

    pub fn add_attendee(mut self, attendee: Attendee) -> Self {
        self.attendees.add(&attendee).expect("Failed to add attendee");
        self
    }
}

// Todo - represents VTODO
#[model]
pub struct Todo {
    pub id: AutoPk<i64>,
    pub calendar: ForeignKey<Calendar>,
    pub uid: String,          // UID property (required)
    pub dtstamp: DateTime<Utc>, // DTSTAMP property (required)
    pub dtstart: Option<DateTime<Utc>>, // DTSTART property
    pub due: Option<DateTime<Utc>>, // DUE property
    pub completed: Option<DateTime<Utc>>, // COMPLETED property
    pub duration: Option<String>, // DURATION property
    pub summary: Option<String>, // SUMMARY property
    pub description: Option<String>, // DESCRIPTION property
    pub location: Option<String>, // LOCATION property
    pub status: Option<String>, // STATUS property
    pub priority: Option<i32>, // PRIORITY property
    pub percent_complete: Option<i32>, // PERCENT-COMPLETE property
    pub organizer: Option<String>, // ORGANIZER property
    pub class: Option<String>, // CLASS property
    pub categories: Option<String>, // CATEGORIES property
    pub created: Option<DateTime<Utc>>, // CREATED property
    pub last_modified: Option<DateTime<Utc>>, // LAST-MODIFIED property
    pub sequence: i32,        // SEQUENCE property
    pub rrule: Option<String>, // RRULE property
    pub alarms: Many<Alarm>,  // VALARM components
    pub attendees: Many<Attendee>, // ATTENDEE properties
}

impl Todo {
    pub fn new(calendar: &Calendar, uid: String) -> Self {
        Todo {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            uid,
            dtstamp: Utc::now(),
            dtstart: None,
            due: None,
            completed: None,
            duration: None,
            summary: None,
            description: None,
            location: None,
            status: None,
            priority: None,
            percent_complete: None,
            organizer: None,
            class: None,
            categories: None,
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            sequence: 0,
            rrule: None,
            alarms: Many::default(),
            attendees: Many::default(),
        }
    }

    // Add a comprehensive constructor with all fields
    pub fn new_with_details(
        calendar: &Calendar,
        uid: String,
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
    ) -> Self {
        Todo {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            uid,
            dtstamp: Utc::now(),
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
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            sequence: 0,
            rrule,
            alarms: Many::default(),
            attendees: Many::default(),
        }
    }

    // Builder pattern methods for fluent API
    pub fn with_dtstart(mut self, dtstart: DateTime<Utc>) -> Self {
        self.dtstart = Some(dtstart);
        self
    }

    pub fn with_due(mut self, due: DateTime<Utc>) -> Self {
        self.due = Some(due);
        self
    }

    pub fn with_completed(mut self, completed: DateTime<Utc>) -> Self {
        self.completed = Some(completed);
        self
    }

    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_percent_complete(mut self, percent_complete: i32) -> Self {
        self.percent_complete = Some(percent_complete);
        self
    }

    pub fn with_organizer(mut self, organizer: impl Into<String>) -> Self {
        self.organizer = Some(organizer.into());
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn with_categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    pub fn with_rrule(mut self, rrule: impl Into<String>) -> Self {
        self.rrule = Some(rrule.into());
        self
    }

    pub fn with_sequence(mut self, sequence: i32) -> Self {
        self.sequence = sequence;
        self
    }

    // Methods to add alarms and attendees
    pub fn add_alarm(mut self, alarm: Alarm) -> Self {
        self.alarms.add(&alarm).expect("Failed to add alarm");
        self
    }

    pub fn add_attendee(mut self, attendee: Attendee) -> Self {
        self.attendees.add(&attendee).expect("Failed to add attendee");
        self
    }
}

// Journal - represents VJOURNAL
#[model]
pub struct Journal {
    pub id: AutoPk<i64>,
    pub calendar: ForeignKey<Calendar>,
    pub uid: String,          // UID property (required)
    pub dtstamp: DateTime<Utc>, // DTSTAMP property (required)
    pub dtstart: Option<DateTime<Utc>>, // DTSTART property
    pub summary: Option<String>, // SUMMARY property
    pub description: Option<String>, // DESCRIPTION property
    pub status: Option<String>, // STATUS property
    pub class: Option<String>, // CLASS property
    pub categories: Option<String>, // CATEGORIES property
    pub created: Option<DateTime<Utc>>, // CREATED property
    pub last_modified: Option<DateTime<Utc>>, // LAST-MODIFIED property
    pub sequence: i32,        // SEQUENCE property
    pub rrule: Option<String>, // RRULE property
}

impl Journal {
    pub fn new(calendar: &Calendar, uid: String) -> Self {
        Journal {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            uid,
            dtstamp: Utc::now(),
            dtstart: None,
            summary: None,
            description: None,
            status: None,
            class: None,
            categories: None,
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            sequence: 0,
            rrule: None,
        }
    }
}

// Alarm - represents VALARM
#[model]
pub struct Alarm {
    pub id: AutoPk<i64>,
    pub action: String,       // ACTION property (required)
    pub trigger: String,      // TRIGGER property (required)
    pub description: Option<String>, // DESCRIPTION property
    pub summary: Option<String>, // SUMMARY property
    pub attendee: Option<String>, // ATTENDEE property
    pub repeat: Option<i32>,  // REPEAT property
    pub duration: Option<String>, // DURATION property
    pub attach: Option<String>, // ATTACH property
}

impl Alarm {
    pub fn new(action: String, trigger: String) -> Self {
        Alarm {
            id: AutoPk::uninitialized(),
            action,
            trigger,
            description: None,
            summary: None,
            attendee: None,
            repeat: None,
            duration: None,
            attach: None,
        }
    }
}

// Attendee - represents ATTENDEE property
#[model]
pub struct Attendee {
    pub id: AutoPk<i64>,
    pub cal_address: String,  // CAL-ADDRESS value
    pub cn: Option<String>,   // CN parameter
    pub cutype: Option<String>, // CUTYPE parameter
    pub member: Option<String>, // MEMBER parameter
    pub role: Option<String>, // ROLE parameter
    pub partstat: Option<String>, // PARTSTAT parameter
    pub rsvp: Option<bool>,   // RSVP parameter
    pub delegated_to: Option<String>, // DELEGATED-TO parameter
    pub delegated_from: Option<String>, // DELEGATED-FROM parameter
}

impl Attendee {
    pub fn new(cal_address: String) -> Self {
        Attendee {
            id: AutoPk::uninitialized(),
            cal_address,
            cn: None,
            cutype: None,
            member: None,
            role: None,
            partstat: None,
            rsvp: None,
            delegated_to: None,
            delegated_from: None,
        }
    }
}

// Timezone - represents VTIMEZONE
#[model]
pub struct Timezone {
    pub id: AutoPk<i64>,
    pub calendar: ForeignKey<Calendar>,
    pub tzid: String,         // TZID property (required)
    pub tzurl: Option<String>, // TZURL property
    pub standard_rules: Many<TimezoneRule>, // STANDARD sub-components
    pub daylight_rules: Many<TimezoneRule>, // DAYLIGHT sub-components
}

impl Timezone {
    pub fn new(calendar: &Calendar, tzid: String) -> Self {
        Timezone {
            id: AutoPk::uninitialized(),
            calendar: calendar.into(),
            tzid,
            tzurl: None,
            standard_rules: Many::default(),
            daylight_rules: Many::default(),
        }
    }
}

// TimezoneRule - represents STANDARD or DAYLIGHT sub-components
#[model]
pub struct TimezoneRule {
    pub id: AutoPk<i64>,
    pub dtstart: DateTime<Utc>, // DTSTART property (required)
    pub tzoffsetfrom: String, // TZOFFSETFROM property (required)
    pub tzoffsetto: String,   // TZOFFSETTO property (required)
    pub rrule: Option<String>, // RRULE property
    pub tzname: Option<String>, // TZNAME property
    pub is_daylight: bool,    // Whether this is a DAYLIGHT or STANDARD rule
}

impl TimezoneRule {
    pub fn new(dtstart: DateTime<Utc>, tzoffsetfrom: String, tzoffsetto: String, is_daylight: bool) -> Self {
        TimezoneRule {
            id: AutoPk::uninitialized(),
            dtstart,
            tzoffsetfrom,
            tzoffsetto,
            rrule: None,
            tzname: None,
            is_daylight,
        }
    }
}

// RecurrenceDate - represents RDATE property
#[model]
pub struct RecurrenceDate {
    pub id: AutoPk<i64>,
    pub event_id: Option<ForeignKey<Event>>,
    pub todo_id: Option<ForeignKey<Todo>>,
    pub date_time: DateTime<Utc>,
}

// ExceptionDate - represents EXDATE property
#[model]
pub struct ExceptionDate {
    pub id: AutoPk<i64>,
    pub event_id: Option<ForeignKey<Event>>,
    pub todo_id: Option<ForeignKey<Todo>>,
    pub date_time: DateTime<Utc>,
}

