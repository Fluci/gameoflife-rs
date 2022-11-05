mod life;
mod steps;
mod world;

pub use life::Life;
use std::io::{stdin, Read};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
pub use steps::step_conway;
pub use world::World;

fn main() {
    let mut life = Life::new_desert(step_conway, 40, 80);

    // Blinker
    life.world.set_cell(4, 5, true);
    life.world.set_cell(5, 5, true);
    life.world.set_cell(6, 5, true);

    // Glider
    life.world.set_cell(4, 8, true);
    life.world.set_cell(5, 9, true);
    life.world.set_cell(3, 10, true);
    life.world.set_cell(4, 10, true);
    life.world.set_cell(5, 10, true);

    let keep_running = Arc::new(RwLock::new(true));
    let check_stop_thread = {
        let keep_running_shared = keep_running.clone();
        thread::spawn(move || {
            check_stop(keep_running_shared);
        })
    };

    clear_screen();
    println!("\n"); // black
    std::thread::sleep(Duration::from_millis(1000));

    loop {
        clear_screen();
        print!("{:?}", life.world);
        println!("Enter: stop");
        std::thread::sleep(Duration::from_millis(200));

        life.step();

        if !*keep_running.read().unwrap() {
            break;
        }
    }

    check_stop_thread.join().unwrap();
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn check_stop(keep_running: Arc<RwLock<bool>>) {
    // block until the user enters something
    stdin().bytes().next();
    *keep_running.write().unwrap() = false;
}
