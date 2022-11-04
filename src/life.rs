use crate::{step_conway, World};

pub struct Life<F>
where F: Fn(bool, usize) -> bool
{
    pub kernel: F,
    pub world: World,
    next_world: World
}

impl<F> Life<F>
where F: Fn(bool, usize) -> bool
{
    pub fn new_desert(kernel: F, height: usize, width: usize) -> Self {
        Life{
            kernel,
            world: World::new_desert(height, width),
            next_world: World::new_desert(height, width),
        }
    }

    pub fn step(&mut self) {
        for row in 0..self.world.get_rows() {
            for col in 0..self.world.get_cols() {
                let was_alive = self.world.get_cell(row, col);
                let living_neighbors = self.world.get_alive_neighbors_count(row, col);
                let is_alive = (self.kernel)(was_alive, living_neighbors);
                self.next_world.set_cell(row, col, is_alive);
            }
        }
        std::mem::swap(&mut self.next_world, &mut self.world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construction() {
        let life = Life::new_desert(step_conway, 10, 8);
        assert_eq!(life.world.get_rows(), 10);
        assert_eq!(life.world.get_cols(), 8);
    }

    #[test]
    fn test_integration() {
        let mut life = Life::new_desert(step_conway, 10, 8);
        life.world.set_cell(4, 5, true);
        life.world.set_cell(5, 5, true);
        life.world.set_cell(6, 5, true);
        life.step();
        assert!(!life.world.get_cell(4, 5));
        assert!(!life.world.get_cell(6, 5));

        assert!(life.world.get_cell(5, 4));
        assert!(life.world.get_cell(5, 5));
        assert!(life.world.get_cell(5, 6));
    }
}

