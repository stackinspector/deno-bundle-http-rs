use std::{str, process::Command};

pub fn handler(path: &str) -> Option<Result<String, String>> {
    let mut path = path.chars();
    match path.next() {
        Some('/') => (),
        _ => return None,
    };
    let path = path.as_str();
    match path {
        "" | "favicon.ico" => return None,
        _ => ()
    }
    let url = format!("https://{}", path);
    let result = Command::new("deno").arg("bundle").arg(url).env("NO_COLOR", "1").output().unwrap();
    let stat = result.status;
    let out = str::from_utf8(&result.stdout[..]).unwrap().to_string();
    let err = str::from_utf8(&result.stderr[..]).unwrap().to_string();
    match stat.success() {
        true => Some(Ok(format!("stdout:\n{}\nstderr:\n{}", out, err))),
        false => Some(Err(format!("stdout:\n{}\nstderr:\n{}", out, err))),
    }
}
