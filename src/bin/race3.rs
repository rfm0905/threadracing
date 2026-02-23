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
                let mut w = x.write().unwrap();
                *w += 5;
                let val = *w;
                println!("write {n}: {val}");
            });
        }

        // readers
        for n in 1..=10 {
            let start = Arc::clone(&start);
            s.spawn(move || {
                start.wait();
                let r = x.read().unwrap();
                let val = *r;
                println!("read  {n}: {val}");
            });
        }

        // go
        start.wait();
    });
}
