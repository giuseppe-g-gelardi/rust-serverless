use dotenv;
use supabase_rust::Supabase;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, serde::Deserialize)]
struct UserRegistration {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    dotenv::dotenv().ok();
    let supabase_url = dotenv::var("NEXT_PUBLIC_SUPABASE_URL").unwrap();
    let supabase_key = dotenv::var("SUPABASE_ANON_KEY").unwrap();
    let supabase = Supabase::new(Some(&supabase_url), Some(&supabase_key), None);

    let req_body = req.into_body();
    let binary_str = String::from_utf8_lossy(&req_body);
    let u: UserRegistration = serde_json::from_str(&binary_str).unwrap();

    match supabase
        .signup_email_password(&u.email.to_string(), &u.password.to_string())
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                let res = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body("OK".into())?;

                return Ok(res);
            } else {
                let res = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body("Bad Request".into())?;

                return Ok(res);
            }
        }
        Err(err) => {
            println!("ERRRRRRR: {:?}", err);
        }
    }

    // let res = Response::builder()
    //     .status(StatusCode::OK)
    //     .header("Content-Type", "application/json")
    //     .body("OK".into())?;
    //
    // Ok(res)

    let res = Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "application/json")
        .body("Internal Server Error".into())?;

    Ok(res)
}

// let req_body = req.into_body();
// let binary_str = String::from_utf8_lossy(&req_body);
// let t: Test = serde_json::from_str(&binary_str).unwrap();
//
// let supabase = Postgrest::new(dotenv::var("SUPABASE_URL").unwrap())
//     .insert_header("apikey", dotenv::var("SUPABASE_ANON_KEY").unwrap());
//
// let res_body = supabase
//     .from("test")
//     .insert(
//         json!({
//             "message": t.message.to_string()
//         })
//         .to_string(),
//     )
//     .execute()
//     .await?;
//
// let ibody = res_body.text().await?;
//
// let res = Response::builder()
//     .status(StatusCode::OK)
//     .header("Content-Type", "application/json")
//     .body(ibody.into())?;
