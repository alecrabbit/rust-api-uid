#![deny(warnings)]

use warp::Filter;
// use uuid::Uuid;
use ulid::Ulid;
// use uuid::v1::{Timestamp, Context};
// use std::time::{SystemTime};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[derive(Deserialize, Serialize)]
struct UID {
    data: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // let promote = warp::get()
    //     .and(warp::path!("uid" / "ulid" ))
    //     .map(|| {
    //         let u = UID { data:  Ulid::new().to_string() };
    //         warp::reply::json(&u)
    //     });
    //
    // warp::serve(promote).run(([127, 0, 0, 1], 4040)).await


    let ulid =
        warp::path!("ulid")
            .map(|| {
                        let u = UID { data:  Ulid::new().to_string() };
                        warp::reply::json(&u)
                    });

    let uid =
        warp::path("uid")
            .and(ulid);

    let routes = warp::get().and(uid);

    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
