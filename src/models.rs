use butane::{model, AutoPk, ForeignKey, Many};

#[model]
#[derive(Debug, Default)]
pub struct Blog {
    pub id: AutoPk<i64>,
    pub name: String,
}
impl Blog {
    pub fn new(name: impl Into<String>) -> Self {
        Blog {
            id: AutoPk::uninitialized(),
            name: name.into()
        }
    }
}

#[model]
pub struct Post {
    pub id: AutoPk<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub blog: ForeignKey<Blog>,
    pub tags: Many<Tag>,
    pub byline: Option<String>,
    pub likes: i32,
}
impl Post {
    pub fn new(blog: &Blog, title: String, body: String) -> Self {
        Post {
            id: AutoPk::uninitialized(),
            title,
            body,
            published: false,
            tags: Many::default(),
            blog: blog.into(),
            byline: None,
            likes: 0,
        }
    }
}

#[model]
#[derive(Debug, Default)]
pub struct Tag {
    #[pk]
    pub tag: String,
}
impl Tag {
    pub fn new(tag: impl Into<String>) -> Self {
        Tag { tag: tag.into() }
    }
}

