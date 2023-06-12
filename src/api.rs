use crate::{ApiList, Item};
use gloo_net::http::{Method, Request, Response};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

// FIXME: should send a Result, because, you know, sometimes people makes mistake when typing their
// password
pub async fn login_attempt(login: &str, password: &str) -> Tokens {
    Request::post(&format!("{}/auth/login", dotenvy_macro::dotenv!("HOST")))
        .json(&LoginAttempt {
            login: login.to_owned(),
            password: password.to_owned(),
        })
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Tokens>()
        .await
        .unwrap()
}

#[derive(Clone, Debug)]
pub struct AuthenticatedClient {
    host: String,
    tokens: RwSignal<Option<Tokens>>,
}

impl AuthenticatedClient {
    pub fn new(tokens: RwSignal<Option<Tokens>>) -> AuthenticatedClient {
        // The HOST of the API is read from environment at compile time
        AuthenticatedClient {
            host: dotenvy_macro::dotenv!("HOST").to_owned(),
            tokens,
        }
    }

    // FIXME: pls Result
    pub async fn fetch_items(&self, page: u32, size: u32) -> Result<ApiList<Item>> {
        //Delegate the call to the internal send method
        let response = self
            .send(
                Method::GET,
                &format!("{}/items?page={}&size={}", &self.host, page, size),
            )
            .await?;
        Ok(response.json::<ApiList<Item>>().await.map_err(|_| FetchError::Json))?
    }

    /// Our call wrapper. If a 401 happens during the call, call the refresh method to obtain a
    /// newer access_token, and try the call again
    /// FIXME: PLS. USE. RESULT.
    async fn send(&self, method: Method, path: &str) -> Result<Response> {
        if let Some(tokens) = self.tokens.get() {
            let response = Request::new(path)
                .method(method)
                .header("Authorization", &format!("Bearer {}", tokens.access_token))
                .send()
                .await
                .map_err(|_| FetchError::Request)?;

            // Token is probably expired, time to get a new token
            if response.status() == 401 {
                let new_access_token = self.refresh_token(&tokens.refresh_token).await;
                self.tokens.update(|x| {
                    let refresh_token = tokens.refresh_token;
                    let access_token = new_access_token.access_token;
                    let new_token = Tokens {
                        access_token,
                        refresh_token,
                    };

                    *x = Some(new_token);
                });

                return Ok(Request::new(path)
                    .method(method)
                    .header(
                        "Authorization",
                        &format!("Bearer {}", self.tokens.get().unwrap().access_token),
                    )
                    .send()
                    .await
                    .map_err(|_| FetchError::Request))?;
            }

            Ok(response)
        } else {
            // FIXME: NO. NOT PANIC. RESULT
            panic!("no tokens");
        }
    }

    // FIXME: Result.
    async fn refresh_token(&self, token: &str) -> Token {
        Request::post(&format!("{}/auth/refresh", self.host))
            .json(&json!({"token": token}))
            .unwrap()
            .send()
            .await
            .unwrap()
            .json::<Token>()
            .await
            .unwrap()
    }
}

type Result<T> = std::result::Result<T, FetchError>;

#[derive(Error, Clone, Debug)]
pub enum FetchError {
    #[error("Error loading data from serving.")]
    Request,
    #[error("Error deserializaing cat data from request.")]
    Json,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub access_token: String,
}

#[derive(Serialize, Debug)]
struct LoginAttempt {
    login: String,
    password: String,
}
