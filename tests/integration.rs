extern crate reql;
extern crate thinker;

use reql::*;
use thinker::r;

#[test]
fn connection_pool_works() {
    r.connect(ConnectOpts::default()).unwrap();
    //let _ = r.table("users").run().unwrap();

    use std::thread;

    let mut children = vec![];
    for i in 0..20 {
        children.push(thread::spawn(move || {
            //let _ = r.db("mufr").table("users").run().unwrap();
            let _ = r.db("blog").table("users").insert(
                r.object()
                .insert("name", format!("User {}", i))
                .insert("age", i*2)
                .build()
                ).run().unwrap();
        }))
    }

    for child in children {
        let _ = child.join();
    }
}
