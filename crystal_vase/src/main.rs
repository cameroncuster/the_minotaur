use rand::Rng;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const N: usize = 10;

    let line = Arc::new(Mutex::new(VecDeque::new()));
    let cnt_visited = Arc::new(AtomicUsize::new(0));
    let visited = Arc::new((0..10).map(|_| AtomicBool::new(false)).collect::<Vec<_>>());

    for guest in 0..N {
        let line = line.clone();
        let cnt_visited = cnt_visited.clone();
        let visited = visited.clone();
        thread::spawn(move || loop {
            if line.lock().unwrap().back() == Some(&guest) {
                let has_visited = visited[guest].load(Ordering::Relaxed);
                if !has_visited {
                    cnt_visited.fetch_add(1, Ordering::Relaxed);
                    visited[guest].store(true, Ordering::Relaxed);
                    println!("Guest {} is visiting the vase for the first time!", guest);
                }

                // allow a new guest to visit the vase
                line.lock().unwrap().pop_back();
            }
        });
    }

    let mut rng = rand::thread_rng();
    while cnt_visited.load(Ordering::Relaxed) < N {
        let guest = rng.gen_range(0..N);
        line.lock().unwrap().push_back(guest);
    }

    println!("All guests have visited the vase!");
}
