use dotenv;
use postgrest::Postgrest;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    dotenv::dotenv().ok();

    let req_body = req.into_body();
    let binary_str = String::from_utf8_lossy(&req_body);
    let t: Test = serde_json::from_str(&binary_str).unwrap();

    let supabase = Postgrest::new(dotenv::var("SUPABASE_URL").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_ANON_KEY").unwrap());

    let res_body = supabase
        .from("test")
        .insert(
            json!({
                "message": t.message.to_string()
            })
            .to_string(),
        )
        .execute()
        .await?;

    let ibody = res_body.text().await?;

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(ibody.into())?;

    Ok(res)
}

#[derive(Debug, serde::Deserialize)]
struct Test {
    message: String,
}
