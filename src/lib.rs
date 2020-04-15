use std::cmp::min;

#[derive(Debug, PartialEq)]
struct GameOfLife(Vec<Vec<bool>>);

impl GameOfLife {
    fn current_state(&self) -> &Vec<Vec<bool>> {
        let GameOfLife(state) = self;
        state
    }

    fn next_state(&self) -> GameOfLife {
        fn is_alive_next(currently_alive: bool, living_neighbor_count: usize) -> bool {
            /*
             * https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules
             * Any live cell with two or three live neighbors survives.
             * Any dead cell with three live neighbors becomes a live cell.
             * All other live cells die in the next generation. Similarly, all other dead cells stay dead.
             */
            currently_alive && living_neighbor_count == 2 || living_neighbor_count == 3
        }

        GameOfLife(
            self.current_state()
                .iter()
                .enumerate()
                .map(|(row_num, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(col_num, currently_alive)| {
                            let living_neighbor_count =
                                self.count_living_neighbors(row_num, col_num);
                            is_alive_next(*currently_alive, living_neighbor_count)
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn count_living_neighbors(&self, row_num: usize, col_num: usize) -> usize {
        // saturating_sub will quietly avoid going lower than 0
        let min_row = row_num.saturating_sub(1);
        let min_col = col_num.saturating_sub(1);
        let max_row = min(self.current_state().len() - 1, row_num + 1);
        (min_row..=max_row)
            .flat_map(|neighbor_row_num| {
                let neighbor_row = &self.current_state()[neighbor_row_num];
                let max_col = min(neighbor_row.len() - 1, col_num + 1);
                (min_col..=max_col).filter(move |&neighbor_col_num| {
                    let current_cell = neighbor_row_num == row_num && neighbor_col_num == col_num;
                    !current_cell && neighbor_row[neighbor_col_num]
                })
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        assert_static(&new_game(&[]));
    }

    #[test]
    fn test_1x1_dead_is_static() {
        assert_static(&new_game(&[" "]));
    }

    #[test]
    fn test_1x1_alive_dies() {
        #[rustfmt::skip]
        assert_next_state(
            &new_game(&["•"]),
            &new_game(&[" "])
        );
    }

    #[test]
    fn test_2x2_block_is_static() {
        #[rustfmt::skip]
        assert_static(&new_game(&[
            "••",
            "••",
        ]));

        #[rustfmt::skip]
        assert_static(&new_game(&[
            "    ",
            " •• ",
            " •• ",
            "    ",
        ]));
    }

    #[test]
    fn test_beehive_is_static() {
        #[rustfmt::skip]
        assert_static(&new_game(&[
            "      ",
            "  ••  ",
            " •  • ",
            "  ••  ",
            "      ",
        ]));
    }

    #[test]
    fn test_tub_is_static() {
        #[rustfmt::skip]
        assert_static(&new_game(&[
            "     ",
            "  •  ",
            " • • ",
            "  •  ",
            "     ",
        ]));
    }

    #[test]
    fn test_blinker_oscillates() {
        #[rustfmt::skip]
        let vertical = new_game(&[
            "     ",
            "  •  ",
            "  •  ",
            "  •  ",
            "     ",
        ]);
        #[rustfmt::skip]
        let horizontal = new_game(&[
            "     ",
            "     ",
            " ••• ",
            "     ",
            "     ",
        ]);
        assert_next_state(&vertical, &horizontal);
        assert_next_state(&horizontal, &vertical);
    }

    #[test]
    fn test_beacon_oscillates() {
        #[rustfmt::skip]
        let on = new_game(&[
            "      ",
            " ••   ",
            " ••   ",
            "   •• ",
            "   •• ",
        ]);
        #[rustfmt::skip]
        let off = new_game(&[
            "      ",
            " ••   ",
            " •    ",
            "    • ",
            "   •• ",
        ]);
        assert_next_state(&on, &off);
        assert_next_state(&off, &on);
    }

    fn new_game(initial_state: &[&str]) -> GameOfLife {
        GameOfLife(
            initial_state
                .iter()
                .map(|row| row.chars().map(|char| char != ' ').collect())
                .collect(),
        )
    }

    fn assert_next_state(current: &GameOfLife, expected_next: &GameOfLife) {
        assert_eq!(current.next_state(), *expected_next);
    }

    fn assert_static(game: &GameOfLife) {
        assert_next_state(game, game);
    }
}
