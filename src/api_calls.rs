// use reqwest::header::{HeaderMap, AUTHORIZATION};
// use std::env;

// #[tokio::main]
// pub async fn get_canvas_data(
//     path: &str,
//     json_holder: DataTypes,
// ) -> Result<Response, reqwest::Error> {
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         AUTHORIZATION,
//         format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
//             .parse()
//             .unwrap(),
//     );
//     let resp = reqwest::Client::new()
//         .get(path)
//         .headers(headers)
//         .send()
//         .await?;

//     let data_vector = resp.json::<Vec<DataTypes>>().await?;
//     Ok(data_vector)
// }
