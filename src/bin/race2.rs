use std::sync::RwLock;
use std::thread;

fn main() {
    let x = RwLock::new(0);

    thread::scope(|s| {
        let x = &x; // so the move doesn't consume

        // 10 writers
        for n in 1..=10 {
            s.spawn(move || {
                let snapshot = {
                    let mut writer = x.write().unwrap();
                    *writer += 5;
                    *writer
                };
                println!("Thread write {n}: {snapshot}");
            });
        }

        // 10 readers
        for n in 1..=10 {
            s.spawn(move || {
                let snapshot = {
                    let reader = x.read().unwrap();
                    *reader
                };
                println!("Thread read {n}: {snapshot}");
            });
        }

        // interleaved spawns
        for n in 10..=20 {
            s.spawn(move || {
                let snapshot = {
                    let mut w = x.write().unwrap();
                    *w += 5;
                    *w
                };
                println!("write {n}: {snapshot}");
            });

            s.spawn(move || {
                let snapshot = {
                    let r = x.read().unwrap();
                    *r
                };
                println!("read  {n}: {snapshot}");
            });
        }
    });
}
