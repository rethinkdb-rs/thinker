extern crate reql;
extern crate thinker;

use reql::*;
use thinker::r;

use std::thread;

#[test]
fn connection_pool_works() {
    let pool = r.connect(ConnectOpts::default()).unwrap();

    let mut children = vec![];

    for _ in 0..5 {
        let pool = pool.clone();
        children.push(thread::spawn(move || {
            let _ = pool.get().unwrap();
        }))
    }

    for child in children {
        let _ = child.join();
    }
}
