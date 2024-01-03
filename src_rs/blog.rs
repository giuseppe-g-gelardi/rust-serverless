pub mod blog {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlogPost {
        pub id: String,
        pub title: String,
        pub content: String,
        pub author: String, // essentially user?
        pub date: String,
        pub is_published: bool,
    }

    impl BlogPost {
        pub fn new(
            id: String,
            title: String,
            content: String,
            author: String,
            date: String,
            is_published: bool,
        ) -> Self {
            Self {
                id,
                title,
                content,
                author,
                date,
                is_published,
            }
        }
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
