mod board {
    use std::cell::Cell;

    #[derive(Copy, Clone, PartialEq)]
    pub enum Square {
        Empty,
        Black,
        White
    }

    pub struct GameState {
        pub squares_filled: Cell<i32>,
        pub winner: Cell<Square>,
        pub board: Cell<[[Square;3]; 3]>
    }


    pub fn init () -> GameState {
        GameState {
            squares_filled: Cell::new(0),
            winner: Cell::new(Square::Empty),
            board: Cell::new([[Square::Empty; 3]; 3])
        }
    }
}

mod validation {
    use board::Square;
    use board::GameState;

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

    fn check_horizontal (game: GameState) -> GameState {
        let res = game.board.get().iter()
            .map(|&r| validate(r))
            .filter(|&cell| cell != Square::Empty)
            .collect::<Vec<Square>>();

        if res.len() > 0 {
            game.winner.set(res[0]);
        }
        return game;
    }

    fn check_vertical (game: GameState) -> GameState {
        return game;
    }

    pub fn inspectBoard(game: GameState) -> GameState {
        // across

        // vertical
        // diagonal
        //  two option top left, bottom left
        return game;
    }
}

fn main() {
    let board = board::init();
    println!("Hello, world!");
}