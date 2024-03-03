use http::StatusCode;
use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};

pub async fn options_handler(
    req: Request<hyper::body::Incoming>,
) -> anyhow::Result<Response<Full<Bytes>>> {
    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(Full::default())?;

    tracing::debug!("Response: status={}, url={}", res.status(), req.uri());
    Ok(res)
}
