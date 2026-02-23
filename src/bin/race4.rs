use std::sync::RwLock;
use std::thread;

fn main() {
    let x = RwLock::new(0);

    thread::scope(|s| {
        let x = &x; // so the move doesn't consume

        // 10 writers
        for n in 1..=10 {
            s.spawn(move || {
                let mut writer = x.write().unwrap();
                *writer += 5;
                let val = *writer;
                println!("Thread write {n}: {val}");
            });
        }

        // 10 readers
        for n in 1..=10 {
            s.spawn(move || {
                let reader = x.read().unwrap();
                let val = *reader;
                println!("Thread read {n}: {val}");
            });
        }

        // interleaved spawns
        for n in 10..=20 {
            s.spawn(move || {
                let mut w = x.write().unwrap();
                *w += 5;
                let val = *w;
                println!("write {n}: {val}");
            });

            s.spawn(move || {
                let r = x.read().unwrap();
                let val = *r;
                println!("read  {n}: {val}");
            });
        }
    });
}
