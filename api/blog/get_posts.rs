use rust_serverless::blog::blog::BlogPost;
use serde::Serialize;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
// use warp::Filter;

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let body = req.into_body();

    let blog_posts = generate_blog_posts();
    let published_blog_posts: Vec<&BlogPost> = blog_posts
        .iter()
        .filter(|&blog_post| blog_post.is_published)
        .collect();

    Ok(Response::builder().status(StatusCode::OK).body(Body::from(
        json!({
            "message": "Hello from Rust!",
            "body": published_blog_posts,
        })
        .to_string(),
    ))?)
}

// mock data
fn generate_blog_posts() -> Vec<BlogPost> {
    let blog_posts = vec![
        BlogPost {
            id: "1".to_string(),
            title: "My first blog post".to_string(),
            content: "This is my first blog post".to_string(),
            author: "John Doe".to_string(),
            date: "2020-01-01".to_string(),
            is_published: true,
        },
        BlogPost {
            id: "2".to_string(),
            title: "My second blog post".to_string(),
            content: "This is my second blog post".to_string(),
            author: "John Doe".to_string(),
            date: "2020-01-02".to_string(),
            is_published: true,
        },
        BlogPost {
            id: "3".to_string(),
            title: "My third blog post".to_string(),
            content: "This is my third blog post".to_string(),
            author: "John Doe".to_string(),
            date: "2020-01-03".to_string(),
            is_published: false,
        },
        BlogPost {
            id: "4".to_string(),
            title: "My fourth blog post".to_string(),
            content: "This is my fourth blog post".to_string(),
            author: "John Doe".to_string(),
            date: "2020-01-04".to_string(),
            is_published: true,
        },
    ];
    return blog_posts;
}
