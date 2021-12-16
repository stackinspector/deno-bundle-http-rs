use std::thread::spawn;
use structopt::StructOpt;
use tiny_http::{Response, Server, Method};
use deno_bundle_http::handler;

#[derive(StructOpt)]
#[structopt(name = "deno-bundle-http")]
struct Args {
    #[structopt(short = "p", long, default_value = "8080")]
    port: u16,
}

fn main() {
    let args = Args::from_args();
    let server = Server::http(format!("0.0.0.0:{}", args.port)).unwrap();

    for request in server.incoming_requests() {
        spawn(move || {
            let method = request.method();
            let path = request.url();
            if method != &Method::Get { request.respond(Response::empty(404)).unwrap() }
            match handler(path) {
                None => request.respond(Response::empty(404)).unwrap(),
                Some(Ok(out)) => request.respond(Response::from_string(out)).unwrap(),
                Some(Err(err)) => request.respond(Response::from_string(err).with_status_code(400)).unwrap(),
            }
        });
    }
}
