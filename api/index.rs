use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body("<h1>you are in index.rs</h1>".to_string().into())?)
}

// let ok = {
//     method: GET,
//     uri: https://localhost:3000/api/index,
//     version: HTTP/1.1,
//     headers: {
//         "host": "localhost:3000",
//         "user-agent": "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
//         "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
//         "accept-language": "en-US,en;q=0.5",
//         "accept-encoding": "gzip, deflate, br",
//         "connection": "close",
//         "upgrade-insecure-requests": "1",
//         "sec-fetch-dest": "document",
//         "sec-fetch-mode": "navigate",
//         "sec-fetch-site": "none",
//         "sec-fetch-user": "?1",
//         "x-real-ip": "::ffff:127.0.0.1",
//         "x-vercel-deployment-url": "localhost:3000",
//         "x-vercel-forwarded-for": "::ffff:127.0.0.1",
//         "x-vercel-id": "dev1::dev1::66k7o-1704134295579-8d46d2451820",
//         "x-forwarded-host": "localhost:3000",
//         "x-forwarded-proto": "http",
//         "x-forwarded-for": "::ffff:127.0.0.1"
//     },
//     body: Binary([])
// };
