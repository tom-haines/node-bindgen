
use std::time::Duration;

use flv_future_aio::timer::sleep;
use node_bindgen::derive::node_bindgen;


#[node_bindgen]
async fn hello<F: Fn(f64,String)>( seconds: i32, cb: F) {

    //println!("sleeping");
    sleep(Duration::from_secs(seconds as u64)).await;
    //println!("woke from time");

    cb(10.0,"hello world".to_string());

}
