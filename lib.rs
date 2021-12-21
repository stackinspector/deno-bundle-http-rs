use std::process::{Command, Output};
use warp::{http::Response, hyper::Body, path::FullPath};

pub async fn handle(url: FullPath) -> impl warp::Reply {
    let Output { status, stdout, stderr } = {
        Command::new("deno")
            .arg("bundle")
            .arg(match url.as_str() {
                "/" | "/favicon.ico" => {
                    return Response::builder().status(404).body(Body::empty()).unwrap()
                }
                _url => format!("https:/{}", _url),
            })
            .env("NO_COLOR", "1")
            .output()
            .unwrap()
    };
    if status.success() {
        Response::builder()
            .status(200)
            .header("content-type", "application/javascript; charset=utf-8")
            .body(Body::from(String::from_utf8(stdout).unwrap()))
            .unwrap()
    } else {
        assert_eq!(stdout.len(), 0);
        Response::builder()
            .status(500)
            .body(Body::from(String::from_utf8(stderr).unwrap()))
            .unwrap()
    }
}
