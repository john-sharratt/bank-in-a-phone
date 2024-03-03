use serde::Serialize;

pub const API_URI: &'static str = if cfg!(debug_assertions) {
    "http://127.0.0.1:8000"
} else {
    "https://immutable-bank.com"
};

#[cfg(not(target_arch = "wasm32"))]
pub async fn post<T: Serialize + 'static>(path: String, data: T) -> anyhow::Result<bytes::Bytes> {
    let url = reqwest::Url::parse(&format!("{}/{}", API_URI, path)).unwrap();

    let response = reqwest::Client::builder()
        .build()?
        .post(url)
        .json(&data)
        .send()
        .await?;

    Ok(response.error_for_status()?.bytes().await?)
}

#[cfg(target_arch = "wasm32")]
pub async fn post<T: Serialize + 'static>(path: String, data: T) -> anyhow::Result<bytes::Bytes> {
    let url = format!("{}/{}", API_URI, path);

    let response = gloo_net::http::Request::post(&url)
        .json(&data)?
        .send()
        .await?;

    if !response.ok() {
        return Err(anyhow::anyhow!("{}", response.status_text()));
    }
    response
        .binary()
        .await
        .map_err(|err| anyhow::anyhow!("{}", err))
        .map(bytes::Bytes::from)
}
