#![deny(warnings)]

use warp::Filter;
use uuid::Uuid;
use ulid::Ulid;
// use uuid::v1::{Timestamp, Context};
// use std::time::{SystemTime};
use serde_derive::{Serialize};

#[derive(Serialize)]
struct Response<T> {
    data: T,
}

#[derive(Serialize)]
struct UlidUID {
    ulid: String,
}

#[derive(Serialize)]
struct UuidV4UID {
    uuid: String,
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
                let r = Response {
                    data: UlidUID {
                        ulid: Ulid::new().to_string()
                    }
                };
                warp::reply::json(&r)
            });

    let uuid_v4 =
        warp::path!("uuid" / "v4")
            // map(|| format!("{}", Uuid::new_v4().to_hyphenated().to_string()));
            .map(|| {
                let r =
                    Response {
                        data: UuidV4UID {
                            uuid: Uuid::new_v4().to_hyphenated().to_string()
                        }
                    };
                warp::reply::json(&r)
            });
    let uid = warp::path("uid").and(
        ulid
            .or(uuid_v4)
        // .or(uuid_v1)
    );

    let routes = warp::get().and(uid);

    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
