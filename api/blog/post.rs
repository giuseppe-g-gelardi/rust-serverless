use rust_serverless::blog::blog::BlogPost;
use serde::Serialize;
use serde_json::{from_slice, json};
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

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // let cors = warp::cors()
//     //     .allow_any_origin()
//     //     .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
//     //     .allow_headers(vec!["Content-Type", "Authorization"]);
//     //
//     // let routes = warp::path!("api" / "blog" / "post")
//     //     .and(warp::post())
//     //     .and(warp::body::json())
//     //     .map(|blog_post: BlogPost| {
//     //         println!("Parsed BlogPost: {:?}", blog_post);
//     //         warp::reply::json(&blog_post)
//     //     });
//     // warp::serve(routes.with(cors))
//     //     .run(([127, 0, 0, 1], 8000))
//     //     .await;
//     //
//     run(handler).await;
// }
//
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Extract the body from the request
    let body = req.into_body();

    // Match the body variant and handle accordingly
    match body {
        Body::Empty => {
            // Return a bad request response if the body is empty
            // Ok(bad_request("Empty body"))
            Ok(Response::builder()
                .status(StatusCode::OK)
                // .header("Content-Type", "application/json")
                // .header("Access-Control-Allow-Origin", "*")
                // .header("Access-Control-Allow-Methods", "*")
                // .header("Access-Control-Allow-Headers", "*")
                // .header("Access-Control-Allow-Credentials", "true")
                .body(
                    json!({
                      "message": "Empty body",
                    })
                    .to_string()
                    .into(),
                )?)
        }
        Body::Text(text) => {
            // Deserialize the JSON body into the BlogPostInput struct
            let blog_post_input: Result<BlogPost, _> = from_slice(text.as_bytes());
            match blog_post_input {
                Ok(input) => {
                    // Use the deserialized input to create a new BlogPost
                    let new_blog_post = BlogPost::new(
                        input.title,
                        input.content,
                        input.author,
                        input.date,
                        input.is_published,
                    );

                    // Return a response with the new blog post information
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        // .header("Content-Type", "application/json")
                        // .header("Access-Control-Allow-Origin", "*")
                        // .header("Access-Control-Allow-Methods", "*")
                        // .header("Access-Control-Allow-Headers", "*")
                        // .header("Access-Control-Allow-Credentials", "true")
                        .body(
                            json!({
                              "message": format!("{:?}!", new_blog_post),
                            })
                            .to_string()
                            .into(),
                        )?)
                }
                Err(_) => {
                    // Return a bad request response if JSON parsing fails
                    // Ok(bad_request("Invalid JSON body"))
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        // .header("Content-Type", "application/json")
                        // .header("Access-Control-Allow-Origin", "*")
                        // .header("Access-Control-Allow-Methods", "*")
                        // .header("Access-Control-Allow-Headers", "*")
                        // .header("Access-Control-Allow-Credentials", "true")
                        .body(
                            json!({
                              "message": "Invalid JSON body",
                            })
                            .to_string()
                            .into(),
                        )?)
                }
            }
        }
        Body::Binary(binary) => {
            let binary_str = String::from_utf8_lossy(&binary);

            // Parse the JSON string into a BlogPost struct
            let blog_post: Result<BlogPost, _> = serde_json::from_str(&binary_str);

            match blog_post {
                Ok(blog_post) => {
                    // Log the parsed struct
                    println!("Parsed BlogPost: {:?}", blog_post);

                    // Now you can use the parsed struct to insert data into a database
                    // ...

                    // Return a response indicating success
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        // .header("Content-Type", "application/json")
                        // .header("Access-Control-Allow-Origin", "*")
                        // .header("Access-Control-Allow-Methods", "*")
                        // .header("Access-Control-Allow-Headers", "*")
                        // .header("Access-Control-Allow-Credentials", "true")
                        .body(
                            // json!({"message": "Blog post received and parsed"})
                            // json!({"message": format!("{:?}", blog_post)})
                            // .to_string()
                            // .into(),
                            { serde_json::to_string(&blog_post)? }.to_string().into(),
                        )?)
                }
                Err(e) => {
                    // Handle the case where parsing fails
                    println!("Error parsing JSON: {:?}", e);
                    Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(json!({"error": "Invalid JSON format"}).to_string().into())?)
                }
            }
        }
    }
}
