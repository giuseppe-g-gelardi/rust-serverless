use dotenv;
use supabase_rust::Supabase;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
struct UserRegistration {
    email: String,
    password: String,
}

pub struct SBUser {
    pub id: String,          // UUID :|
    pub instance_id: String, // also a UUID :|
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

    println!("Request: {:?}", req.method());
    if req.method() == "OPTIONS" {
        let res = Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*") // or specify the allowed origins
            .header("Access-Control-Allow-Methods", "OPTIONS, POST")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::Empty)?;
        return Ok(res);
    }

    let req_body = req.into_body();
    let binary_str = String::from_utf8_lossy(&req_body);
    // let u: UserRegistration = serde_json::from_str(&binary_str).unwrap();
    let u: Result<UserRegistration, _> = serde_json::from_str(&binary_str);
    println!("User: {:?}", u);

    // Improve error handling for JSON parsing
    let u = match u {
        Ok(u) => u,
        Err(err) => {
            let err_res = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Failed to parse JSON: {}", err)))?;
            return Ok(err_res);
        }
    };

    match supabase
        .sign_in_password(&u.email.to_string(), &u.password.to_string())
        .await
    {
        Ok(res) => {
            // Check if the response status is OK
            if res.status().is_success() {
                // Parse the response body to extract tokens
                let json_response: serde_json::Value = res.json().await.unwrap();
                println!("SUPABASE json Response: {:?}", json_response);

                if let Some(access_token) =
                    json_response.get("access_token").and_then(|t| t.as_str())
                {
                    // Sign-in was successful, access_token is available
                    println!("Access Token: {}", access_token);
                    let success_res = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Allow-Origin", "*") // or specify the allowed origins
                        .header("Set-Cookie", format!("access_token={}", access_token)) // sb-access-token ??
                        .body(json!({ "access_token": access_token }).to_string().into())?;
                    // You may want to do something with the access token, such as storing it securely

                    return Ok(success_res);
                } else {
                    println!("Access token not found in the response.");
                }
                // refresh token?? // ... (handle successful sign-in)
            } else {
                // Handle unsuccessful sign-in
                println!("Sign In Failed. Status: {:?}", res.status());
                // ... (handle unsuccessful sign-in)
            }
        }
        Err(err) => {
            println!("Error signing in: {:?}", err);
            // ... (handle sign-in error)
            // Improve error handling for sign-in errors
            let err_res = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Failed to parse JSON: {}", err)))?;
            return Ok(err_res);
        }
    }

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body("OK".into())?;

    Ok(res)
}

//
// //                if let Some(refresh_token) =
//                     json_response.get("refresh_token").and_then(|t| t.as_str())
//                 {
//                     // Refresh token is available (optional)
//                     println!("Refresh Token: {}", refresh_token);
//                     // You may want to do something with the refresh token
//                 } else {
//                     println!("Refresh token not found in the response.");
//                 }
//
