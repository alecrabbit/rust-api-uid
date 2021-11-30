#![deny(warnings)]

use warp::Filter;
use uuid::Uuid;
use ulid::Ulid;
// use uuid::v1::{Timestamp, Context};
// use std::time::{SystemTime};
use serde_derive::{Serialize};

#[derive(Serialize)]
struct UlidUID {
    data: String,
}

#[derive(Serialize)]
struct UuidV4UID {
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
                let u = UlidUID { data: Ulid::new().to_string() };
                warp::reply::json(&u)
            });

    let uuid_v4 =
        warp::path!("uuid" / "v4")
            // map(|| format!("{}", Uuid::new_v4().to_hyphenated().to_string()));
            .map(|| {
                let u = UuidV4UID { data: Uuid::new_v4().to_hyphenated().to_string() };
                warp::reply::json(&u)
            });
    let uid = warp::path("uid").and(
        ulid
            .or(uuid_v4)
        // .or(uuid_v1)
    );

    let routes = warp::get().and(uid);

    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
