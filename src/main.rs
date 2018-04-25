mod board {
    #[derive(Copy, Clone)]
    pub enum Square {
        Empty,
        Black,
        White
    }

    pub fn init () -> [[Square;3]; 3] {
        [[Square::Empty; 3]; 3]
    }
}

fn main() {
    let board = board::init();
    println!("Hello, world!");
}