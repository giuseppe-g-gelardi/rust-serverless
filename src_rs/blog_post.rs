pub mod blog_posts {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FrontMatter {
        pub id: i8,
        pub title: String,
        pub tags: Vec<String>,
        // image: String,
    }

    pub struct BlogPosts {
        pub id: i8,
        pub front_matter: FrontMatter,
        pub author: String,
        pub content: String,
        pub date: String,
        pub created_at: String,
        pub updated_at: String,
        pub is_published: bool,
        // reacts: i8,
        // comments: Vec<Comments>,
    }
}
/*
define the blog post template with front matter, like this:
    ---
    id: 1704202457-QVNO
    aliases: []
    tags: []
    ---

   or like this:

    ---
    id: 1704202457-QVNO
    title: A trip to Iceland
    author: 'Watson & Crick '
    date: '2019-07-10T16:04:44.000Z'
    hero_image: /norris-niman-iceland.jpg
    tags: [] -- like hashtags
    ---
*/
