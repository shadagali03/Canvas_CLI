use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
#[tokio::main]
pub async fn call_canvas_api<T>(path: &String) -> Result<T, &'static str>
where
    T: serde::de::DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(path.as_str())
        .headers(headers)
        .send()
        .await;
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                let account_info = resp.json::<T>().await.unwrap();
                return Ok(account_info);
            } else {
                return Err("Error getting account info");
            }
        }
        Err(_) => return Err("Error getting account info"),
    }
}

#[tokio::main]
pub async fn post_data_api<T>(
    path: &String,
    form: reqwest::multipart::Form,
) -> Result<T, &'static str>
where
    T: serde::de::DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .post(path.as_str())
        .headers(headers)
        .multipart(form)
        .send()
        .await;
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                let account_info = resp.json::<T>().await.unwrap();
                return Ok(account_info);
            } else {
                return Err("Error getting account info");
            }
        }
        Err(_) => return Err("Error getting account info"),
    }
}
