#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let ulid = warp::path!("ulid").map(|| format!("{}", "ulid"));
    let uuid_v1 = warp::path!("uuid" / "v1").map(|| format!("{} {}", "uuid", "v1"));
    let uuid_v4 = warp::path!("uuid" / "v4").map(|| format!("{} {}", "uuid", "v4"));

    let uid = warp::path("uid").and(ulid.or(uuid_v1).or(uuid_v4));

    let routes = warp::get().and(uid);

    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
