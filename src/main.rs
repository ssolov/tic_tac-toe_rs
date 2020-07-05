use std::cmp;
use std::io;
use std::fmt;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum BoardChar {
    O,
    X,
    Empty,
}

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for BoardChar {
    fn to_char(&self) -> char {
        match *self {
            BoardChar::Empty => ' ',
            BoardChar::O => 'O',
            BoardChar::X => 'X'
        }
    }
}

trait Opposite {
    fn to_opposite(&self) -> BoardChar;
}

impl Opposite for BoardChar {
    fn to_opposite(&self) -> BoardChar {
        match *self {
            BoardChar::Empty => BoardChar::Empty,
            BoardChar::O => BoardChar::X,
            BoardChar::X => BoardChar::O
        }        
    }
}

struct Move { 
    row: usize, 
    col: usize,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row = self.row + 1;
        let col: char = match self.col {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            _ => ' ',
        };

        write!(f, "{}{}", row , col)
    }
}

type Board = [[BoardChar; 3]; 3];

trait BoardMoves {
    /// This function returns true if there are moves remaining on the board. 
    /// It returns false if there are no moves left to play. 
    fn has_moves(&self) -> bool;
    fn move_char(&mut self, m: &Move, c: BoardChar);
    fn evaluate(&self) -> Option<BoardChar>;
    fn find_best_move(&mut self, c: BoardChar) -> Option<Move>;
}

impl BoardMoves for Board {
    fn has_moves(&self) -> bool { 
        for row in 0..3 {
            for col in 0..3 {
                if self[row][col] == BoardChar::Empty {
                    return true;
                } 
            }
        }

        false
    }

    fn move_char(&mut self, m: &Move, c: BoardChar) {
        self[m.row][m.col] = c
    }

    fn evaluate(&self) -> Option<BoardChar> { 
        // Checking for Rows for X or O victory. 
        for row in 0..3 {
            if self[row][0] != BoardChar::Empty && self[row][0] == self[row][1] && self[row][1] == self[row][2] { 
                return Some(self[row][0]);
            } 
        } 
      
        // Checking for Columns for X or O victory. 
        for col in 0..3 {
            if self[0][col] != BoardChar::Empty && self[0][col] == self[1][col] && self[1][col] == self[2][col] { 
                return Some(self[0][col]); 
            } 
        } 
      
        // Checking for Diagonals for X or O victory. 
        if self[0][0] != BoardChar::Empty && self[0][0] == self[1][1] && self[1][1] == self[2][2] {
            return Some(self[0][0]);
        } 
      
        if self[0][2] != BoardChar::Empty && self[0][2] == self[1][1] && self[1][1] == self[2][0] {
            return Some(self[0][2]);
        } 
      
        if self.has_moves() {
            return Some(BoardChar::Empty);
        }

        // Else if none of them have won and no free moves
        None
    }

    fn find_best_move(&mut self, c: BoardChar) -> Option<Move> {
        let mut best_val = -10; 
        let mut best_move = None;
      
        // Traverse all cells, evaluate minimax function for 
        // all empty cells. And return the cell with optimal 
        // value. 
        for i in 0..3 { 
            for j in 0..3 { 
                // Check if cell is empty 
                if self[i][j] == BoardChar::Empty { 
                    // Make the move 
                    let m = Move{row: i, col: j};
                    self.move_char(&m, c);
      
                    // compute evaluation function for this move. 
                    let move_val = minimax(self, c.to_opposite()); 
 
                    // If the value of the current move is 
                    // more than the best value, then update 
                    // best/ 
                    if move_val > best_val { 
                        best_move = Some(Move {row: i, col: j}); 
                        best_val = move_val; 
                    } 

                    // Undo the move 
                    self[i][j] = BoardChar::Empty;
                } 
            } 
        } 
      
        best_move
    }
}

trait PrintBoard {
    /// Prints entire Board.
    fn print(&self);
    /// Prints the row of the Board by row number.
    fn print_row(&self, nr: usize);
}

impl PrintBoard for Board {
    fn print(&self) {
        println!("  A B C\n \u{250C}\u{2500}\u{252C}\u{2500}\u{252C}\u{2500}\u{2510}");
        
        self.print_row(0);
        println!("\n \u{251C}\u{2500}\u{253C}\u{2500}\u{253C}\u{2500}\u{2524}");

        self.print_row(1);
        println!("\n \u{251C}\u{2500}\u{253C}\u{2500}\u{253C}\u{2500}\u{2524}");

        self.print_row(2);
        println!("\n \u{2514}\u{2500}\u{2534}\u{2500}\u{2534}\u{2500}\u{2518}\n");
    }

    fn print_row(&self, nr: usize) {
        print!("{}\u{2502}", nr + 1);

        for row in &self[nr] {
            print!("{}\u{2502}", row.to_char());
        }
    }
}

// This is the minimax function. It considers all the possible ways 
// the game can go and returns the value of the board 
fn minimax(b: &mut Board, c: BoardChar) -> i16 { 
    match b.evaluate() {
        Some(BoardChar::X) => return 1, // If Maximizer has won the game return his/her evaluated score 
        Some(BoardChar::O) => return -1, // If Minimizer has won the game return his/her evaluated score 
        None => return 0,
        _ => ()
    }
  
    let mut best: i16 = if c == BoardChar::X {
        // If this maximizer's move
        -10
    } else {
        // If this minimizer's move 
        10
    };

    for i in 0..3 { 
        for j in 0..3 { 
            // Check if cell is empty 
            if b[i][j] == BoardChar::Empty { 
                // Make the move 
                let m = Move{row: i, col: j};
                b.move_char(&m, c);

                if c == BoardChar::X {
                    // Call minimax recursively and choose the maximum value 
                    best = cmp::max(best, minimax(b, c.to_opposite())); 
                } else {
                    // Call minimax recursively and choose the minimum value 
                    best = cmp::min(best, minimax(b, c.to_opposite())); 
                }
  
                // Undo the move 
                b.move_char(&m, BoardChar::Empty); 
            } 
        } 
    } 
    
    best
}

fn input_to_move(inp: &str) -> Option<Move> {
    if inp.len() == 2 {
       let mut col: Option<usize> = None;
       let mut row: Option<usize> = None;

       for c in inp.chars() {
            match c {
                'A' | 'a' => col = Some(0),
                'B' | 'b' => col = Some(1),
                'C' | 'c' => col = Some(2),
                '1' => row = Some(0),
                '2' => row = Some(1),
                '3' => row = Some(2),
                _ => (),
            }
        }

        if row.is_some() && col.is_some() {
            return Some(Move {row: row.unwrap(), col: col.unwrap()});
        }
    }

    None
}

fn main() {
    let mut b: Board = [[BoardChar::Empty; 3]; 3]; 
    b.print();

    while let Some(BoardChar::Empty) = b.evaluate() {
        println!("your move: ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(m) = input_to_move(input.trim()) {
                    b.move_char(&m, BoardChar::O);
                    b.print();
                    // if let Some(BoardChar::O) = b.evaluate() {
                    //     break;
                    // }

                    if let Some(m) = b.find_best_move(BoardChar::X) {
                        b.move_char(&m, BoardChar::X);
                        println!("machine moved: {}", m);
                        b.print();
                    }
                }
            },
            Err(error) => panic!(error),
        }
    }

    match b.evaluate() {
        Some(c) => println!("won: {}", c.to_char()),
        None => println!("draw"),
    }
}
    