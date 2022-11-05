extern crate num_traits;

mod life;
mod steps;
mod world;

pub use life::Life;
use std::time::Duration;
pub use steps::step_conway;
pub use world::World;

fn main() {
    let mut life = Life::new_desert(step_conway, 40, 40);
    // Blinker
    life.world.set_cell(4, 5, true);
    life.world.set_cell(5, 5, true);
    life.world.set_cell(6, 5, true);

    // Glider

    //life.world.set_cell(3, 8, true);
    life.world.set_cell(4, 8, true);
    life.world.set_cell(5, 9, true);
    life.world.set_cell(3, 10, true);
    life.world.set_cell(4, 10, true);
    life.world.set_cell(5, 10, true);

    print!("{:?}", life.world);
    loop {
        std::thread::sleep(Duration::from_millis(200));
        life.step();
        print!("{}[2J", 27 as char);
        print!("{:?}", life.world);
    }
}
