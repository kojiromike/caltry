use butane::db::{Connection, ConnectionSpec};
use butane::migrations::Migrations;
use butane::prelude::*;
pub mod butane_migrations;
pub mod models;
pub use models::{Blog, Post};

pub fn create_blog(conn: &Connection, name: impl Into<String>) -> Blog {
    let mut blog = Blog::new(name);
    blog.save(conn).unwrap();
    blog
}

pub fn create_post(conn: &Connection, blog: &Blog, title: String, body: String) -> Post {
    let mut new_post = Post::new(blog, title, body);
    new_post.save(conn).unwrap();
    new_post
}

pub fn establish_connection() -> Connection {
    let mut connection = butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap();
    let migrations = butane_migrations::get_migrations().unwrap();
    migrations.migrate(&mut connection).unwrap();
    connection
}

pub fn existing_blog(conn: &Connection) -> Option<Blog> {
    Blog::query().load_first(conn).unwrap()
}
