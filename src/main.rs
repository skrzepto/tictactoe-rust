mod board {
    use std::cell::Cell;

    #[derive(Copy, Clone, PartialEq)]
    pub enum Square {
        Empty,
        Black,
        White
    }

    pub struct GameState {
        pub squares_filled: i32,
        pub winner: Square,
        pub board: [[Square;3]; 3]
    }

    impl GameState {
        fn check_horizontal (&mut self) {
            let res = self.board.iter()
                .map(|&r| validate(r))
                .filter(|&cell| cell != Square::Empty)
                .collect::<Vec<Square>>();

            if res.len() > 0 {
                self.winner = res[0];
            }
        }

        fn check_vertical (game: GameState) {
        }

        pub fn inspectBoard(game: GameState) {
            // across

            // vertical
            // diagonal
            //  two option top left, bottom left
        }
    }

    fn validate(row: [Square;3]) -> Square {
        let non_empty = row.iter()
            .filter(|&&cell| cell != Square::Empty)
            .collect::<Vec<&Square>>();

        if non_empty.len() == 3 {
            let black_elements = row.iter()
                .filter(|&&cell| cell == Square::Black)
                .collect::<Vec<&Square>>();
            if black_elements.len() == 3 {
                return Square::Black;
            } else if black_elements.len() == 0 {
                return Square::White;
            }
        }
        return Square::Empty;
    }

    pub fn init () -> GameState {
        GameState {
            squares_filled: 0,
            winner: Square::Empty,
            board: [[Square::Empty; 3]; 3]
        }
    }
}

mod validation {
    use board::Square;
    use board::GameState;






}

fn main() {
    let board = board::init();
    println!("Hello, world!");
}