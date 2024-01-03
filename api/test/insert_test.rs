use dotenv;
use postgrest::Postgrest;
// use serde::Serialize;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    dotenv::dotenv().ok();

    let client_with_token = Postgrest::new(dotenv::var("SUPABASE_URL").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_ANON_KEY").unwrap());

    let new_row = Test {
        message: "Hello, supabase!".to_string(),
    };

    let ires = client_with_token
        .from("test")
        .insert(
            json!({
                "message": new_row.message
            })
            .to_string(),
        )
        .execute()
        .await?;

    let ibody = ires.text().await?;

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(ibody.into())?;

    Ok(res)
}

struct Test {
    message: String,
}
