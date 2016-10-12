extern crate reql;
extern crate thinker;

use reql::*;
use thinker::r;

//use std::thread;

#[test]
fn connection_pool_works() {
    r.connect(ConnectOpts::default()).unwrap();

    /*
    let mut children = vec![];
    for _ in 0..10000 {
        children.push(thread::spawn(move || {
            let ref pool = r.pool.read().unwrap();
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
    */
}
