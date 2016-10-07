extern crate reql;
extern crate thinker;

use reql::*;
use thinker::r;

use std::thread;

#[test]
fn connection_pool_works() {
    r.connect(ConnectOpts::default());

    let mut children = vec![];
    for _ in 0..5 {
        let pool = r.pool.clone();
        children.push(thread::spawn(move || {
            let mut pool = pool.lock().unwrap();
            match *pool {
                Some(ref p) => { 
                    let _ = p.get().unwrap();
                },
                None => panic!("no connection pool available"),
            };
        }))
    }

    for child in children {
        let _ = child.join();
    }
}
