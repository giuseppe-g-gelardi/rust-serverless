use rust_serverless::req_url_parser;
use serde::Serialize;
use vercel_runtime::{http::bad_request, run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let id_key = req_url_parser(req, "id")?;

    match id_key {
        None => {
            return bad_request(APIError {
                message: "Query string is invalid",
                code: "query_string_invalid",
            });
        }
        Some(id) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::Text(id.to_owned()))?),
    }
}
