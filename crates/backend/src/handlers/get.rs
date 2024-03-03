use http::StatusCode;
use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};
use include_dir::{include_dir, Dir};

const HTML_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../crates/web/dist");

pub async fn get_handler(
    req: Request<hyper::body::Incoming>,
) -> anyhow::Result<Response<Full<Bytes>>> {
    // Get the path to the thing we loading and sanitize it
    let mut path = req.uri().path();

    // Special case and strip slash
    if path.is_empty() || path == "/" {
        path = "/index.html";
    }
    if path.starts_with("/") {
        path = &path[1..];
    }

    // Sanitize
    if path.contains("..") || path.starts_with("/") || path.starts_with("~") {
        tracing::warn!("Access denied: path={}", path);
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Full::new(Bytes::from("Access denied")))?);
    }

    // Load the file
    let file = match HTML_DIR.get_file(path) {
        Some(file) => file,
        None => {
            tracing::debug!("Not found: path={}", path);
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("File not found")))?);
        }
    };

    // Cache time
    let cache_control = if file.path().ends_with(".mp3")
        || file.path().ends_with(".mp4")
        || file.path().ends_with(".wav")
        || file.path().ends_with(".wasm")
    {
        "Cache-Control: max-age=86400, stale-while-revalidate=86400"
    } else {
        "Cache-Control: max-age=30, stale-while-revalidate=86400"
    };

    // Write the response
    let mut res = Response::new(Full::new(Bytes::from(file.contents())));

    let meme = mime_guess::from_path(file.path()).first_or_octet_stream();
    res.headers_mut().insert(
        http::header::CONNECTION,
        http::HeaderValue::from_str("Keep-Alive")?,
    );
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_str(&meme.to_string())?,
    );
    res.headers_mut().insert(
        http::header::CACHE_CONTROL,
        http::HeaderValue::from_str(cache_control)?,
    );

    tracing::debug!("Response: status={}, url={}", res.status(), req.uri());
    Ok(res)
}
