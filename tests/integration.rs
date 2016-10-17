extern crate reql;
extern crate thinker;

use reql::*;
use thinker::r;

#[test]
fn connection_pool_works() {
    //r.connect(ConnectOpts::default()).unwrap();
    let _ = r.table("users").run().unwrap();
    //let _ = r.db("blog").table("users").filter(format!("{}", "{\"name\":\"Michel\"}")).run().unwrap();

    /*
    use std::thread;

    let mut children = vec![];
    for _ in 0..100 {
        children.push(thread::spawn(move || {
            let _ = r.db("mufr").table("users").run().unwrap();
        }))
    }

    for child in children {
        let _ = child.join();
    }
    */
}
