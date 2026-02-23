use std::sync::{Arc, Barrier, RwLock};
use std::thread;

fn main() {
    let x = RwLock::new(0);

    thread::scope(|s| {
        let x = &x; // so the move doesn't consume

        // barrier so all threads start at same time
        let start = Arc::new(Barrier::new(21));

        // writers
        for n in 1..=10 {
            let start = Arc::clone(&start);
            s.spawn(move || {
                start.wait();
                let snapshot = {
                    let mut w = x.write().unwrap();
                    *w += 5;
                    *w
                };
                println!("write {n}: {snapshot}");
            });
        }

        // readers
        for n in 1..=10 {
            let start = Arc::clone(&start);
            s.spawn(move || {
                start.wait();
                let snapshot = {
                    let r = x.read().unwrap();
                    *r
                };
                println!("read  {n}: {snapshot}");
            });
        }

        // go
        start.wait();
    });
}
