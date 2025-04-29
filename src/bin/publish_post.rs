use std::env::args;

use butane::prelude::*;

use caltry::models::Post;
use caltry::*;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let conn = establish_connection();

    let mut post = Post::get(&conn, id).expect(&format!("Unable to find post {}", id));
    // Just a normal Rust assignment, no fancy set methods
    post.published = true;
    post.save(&conn).unwrap();
    println!("Published post {}", post.title);
}
