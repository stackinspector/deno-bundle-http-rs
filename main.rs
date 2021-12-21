use tokio::{spawn, signal, sync::oneshot};
use structopt::StructOpt;
use warp::Filter;
use deno_bundle_http::handle;

#[derive(StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[structopt(short = "p", long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let Args { port } = Args::from_args();

    let (tx, rx) = oneshot::channel();

    let route = warp::get()
        .and(warp::path::full())
        .then(handle);
        // .with(warp::compression::gzip());

    let (_addr, server) = warp::serve(route).bind_with_graceful_shutdown(
        ([127, 0, 0, 1], port),
        async { rx.await.unwrap(); }
    );

    spawn(server);

    signal::ctrl_c().await.unwrap();
    tx.send(()).unwrap();
}
