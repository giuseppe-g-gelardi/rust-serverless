use dotenv;
// use postgrest::Postgrest;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
// use serde_json::json;
use supabase_rust::Supabase;

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
        .sign_in_password(&u.email.to_string(), &u.password.to_string())
        .await
    {
        Ok(res) => {
            println!("Sign In Response: {:?}", res);

            // Check if the response status is OK
            if res.status().is_success() {
                // Parse the response body to extract tokens
                let json_response: serde_json::Value = res.json().await.unwrap();
                if let Some(access_token) =
                    json_response.get("access_token").and_then(|t| t.as_str())
                {
                    // Sign-in was successful, access_token is available
                    println!("Access Token: {}", access_token);
                    // You may want to do something with the access token, such as storing it securely
                } else {
                    println!("Access token not found in the response.");
                }

                if let Some(refresh_token) =
                    json_response.get("refresh_token").and_then(|t| t.as_str())
                {
                    // Refresh token is available (optional)
                    println!("Refresh Token: {}", refresh_token);
                    // You may want to do something with the refresh token
                } else {
                    println!("Refresh token not found in the response.");
                }

                // ... (handle successful sign-in)
            } else {
                // Handle unsuccessful sign-in
                println!("Sign In Failed. Status: {:?}", res.status());
                // ... (handle unsuccessful sign-in)
            }
        }
        Err(err) => {
            println!("Error signing in: {:?}", err);
            // ... (handle sign-in error)
        }
    }

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body("OK".into())?;

    Ok(res)
}

pub struct SBUser {
    pub id: String,          // UUID :|
    pub instance_id: String, // also a UUID :|
}
