use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const N: usize = 10;

    struct Labyrinth {
        have_eaten: Vec<bool>,
        num_guests_who_ate: usize,
        is_cupcake_available: bool,
    }

    let labyrinth = Arc::new(Mutex::new(Labyrinth {
        have_eaten: vec![false; N],
        num_guests_who_ate: 0,
        is_cupcake_available: true,
    }));

    let guest_eating = Arc::new(Mutex::new(N));

    for guest in 0..N {
        let labyrinth = labyrinth.clone();
        let guest_eating = guest_eating.clone();
        thread::spawn(move || loop {
            if *guest_eating.lock().unwrap() == guest {
                let mut labyrinth = labyrinth.lock().unwrap();
                if !labyrinth.have_eaten[guest] {
                    if !labyrinth.is_cupcake_available {
                        // guest requests a cupcake
                        labyrinth.is_cupcake_available = true;
                    }
                    labyrinth.have_eaten[guest] = true;
                    labyrinth.num_guests_who_ate += 1;
                    labyrinth.is_cupcake_available = false;
                    println!("Guest {} has eaten", guest);
                }
            }
        });
    }

    let mut rng = rand::thread_rng();
    while labyrinth.lock().unwrap().num_guests_who_ate < N {
        let guest = rng.gen_range(0..N);
        *guest_eating.lock().unwrap() = guest;
    }

    println!("All guests have visited the labyrinth!");
}
