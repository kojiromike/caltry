use butane::query;

use butane::prelude::QueryOpsSync;

use caltry::models::Post;
use caltry::*;

fn main() {
    let conn = establish_connection();
    let results = query!(Post, published == true)
        .limit(5)
        .load(&conn)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
