pub fn step_conway(is_alive: bool, number_of_alife_neighbors: usize) -> bool {
    if is_alive {
        return 2 == number_of_alife_neighbors || number_of_alife_neighbors == 3;
    }
    return number_of_alife_neighbors == 3;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cell_transition_alive() {
        assert_eq!(step_conway(true, 0), false);
        assert_eq!(step_conway(true, 1), false);
        assert_eq!(step_conway(true, 2), true);
        assert_eq!(step_conway(true, 3), true);
        assert_eq!(step_conway(true, 4), false);
        assert_eq!(step_conway(true, 5), false);
        assert_eq!(step_conway(true, 6), false);
        assert_eq!(step_conway(true, 7), false);
        assert_eq!(step_conway(true, 8), false);
    }

    #[test]
    fn test_cell_transition_dead() {
        assert_eq!(step_conway(false, 0), false);
        assert_eq!(step_conway(false, 1), false);
        assert_eq!(step_conway(false, 2), false);
        assert_eq!(step_conway(false, 3), true);
        assert_eq!(step_conway(false, 4), false);
        assert_eq!(step_conway(false, 5), false);
        assert_eq!(step_conway(false, 6), false);
        assert_eq!(step_conway(false, 7), false);
        assert_eq!(step_conway(false, 8), false);
    }
}
