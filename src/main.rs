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

    let promote = warp::get()
        .and(warp::path!("uid" / "ulid" ))
        .map(|| {
            let u = UID { data:  Ulid::new().to_string() };
            warp::reply::json(&u)
        });

    warp::serve(promote).run(([127, 0, 0, 1], 4040)).await


    // // let context = &Context::new(42);
    // // let node_id = &[1, 2, 3, 4, 5, 6];
    //
    // let ulid = warp::path!("ulid").map(|| format!("{}", Ulid::new().to_string()));
    //
    // // let uuid_v1 =
    // //     warp::path!("uuid" / "v1")
    // //         .map(
    // //             || {
    // //                 let now =
    // //                     match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    // //                         Ok(n) => n,
    // //                         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // //                     };
    // //
    // //                 let ts = Timestamp::from_unix(context, now.as_secs(), now.as_nanos() as u32);
    // //
    // //                 format!(
    // //                     "{}",
    // //                     match Uuid::new_v1(ts, node_id) {
    // //                         Ok(generated) => generated.to_hyphenated().to_string(),
    // //                         Err(_) => format!("{}", "Failed to generate UUID!"),
    // //                     }
    // //                 )
    // //             }
    // //         );
    //
    // let uuid_v4 = warp::path!("uuid" / "v4").map(|| format!("{}", Uuid::new_v4().to_hyphenated().to_string()));
    //
    // let uid = warp::path("uid").and(
    //     ulid
    //         .or(uuid_v4)
    //         // .or(uuid_v1)
    // );
    //
    // let routes = warp::get().and(uid);
    //
    // warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
