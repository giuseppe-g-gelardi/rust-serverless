use dotenv;
use postgrest::Postgrest;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    dotenv::dotenv().ok();

    let client_with_token = Postgrest::new(dotenv::var("SUPABASE_URL").unwrap())
        .insert_header("apikey", dotenv::var("SUPABASE_ANON_KEY").unwrap());
    let tresp = client_with_token.from("test").select("*").execute().await?;
    let tbody = tresp.text().await?;

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(tbody.into())?;

    Ok(res)
}

