use std::cmp;
use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum BoardChar {
    O,
    X,
    Empty,
}

impl BoardChar {
    fn to_opposite(&self) -> BoardChar {
        match *self {
            BoardChar::Empty => BoardChar::Empty,
            BoardChar::O => BoardChar::X,
            BoardChar::X => BoardChar::O,
        }
    }
}

impl fmt::Display for BoardChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BoardChar::Empty => write!(f, " "),
            BoardChar::O => write!(f, "O"),
            BoardChar::X => write!(f, "X"),
        }
    }
}

impl FromStr for BoardChar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tr = s.trim();
        if tr.len() > 1 {
            return Err(format!("Input {} too lang", tr));
        }

        if let Some(c) = tr.chars().next() {
            match c {
                'X' | 'x' => return Ok(BoardChar::X),
                'O' | 'o' => return Ok(BoardChar::O),
                _ => return Err(format!("'{}' is not one of 'X', 'x', 'O', 'o'", c)),
            }
        }

        Err(format!("Could not parse: {}", tr))
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

        write!(f, "{}{}", row, col)
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tr = s.trim();
        if tr.len() != 2 {
            return Err(format!("Input {} too lang", tr));
        }

        let mut col: Option<usize> = None;
        let mut row: Option<usize> = None;

        for c in tr.chars() {
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
            return Ok(Move {
                row: row.unwrap(),
                col: col.unwrap(),
            });
        }

        Err(format!("Could not parse: {}", tr))
    }
}

type Board = [[BoardChar; 3]; 3];

struct TicTacToe {
    board: Board,
    player_char: BoardChar,
    machine_char: BoardChar,
}

impl TicTacToe {
    fn new(player_char: BoardChar) -> TicTacToe {
        TicTacToe {
            board: [[BoardChar::Empty; 3]; 3],
            player_char,
            machine_char: player_char.to_opposite(),
        }
    }

    /// This function returns true if game is over
    fn is_game_over(&self) -> bool {
        !self.has_moves() || self.player_won() || self.machine_won()
    }
    /// This function returns true if there are moves remaining on the board.
    /// It returns false if there are no moves left to play.    
    fn has_moves(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == BoardChar::Empty {
                    return true;
                }
            }
        }

        false
    }
    /// This function makes the player's move
    fn player_move(&mut self, m: &Move) -> bool {
        self.do_move(m, self.player_char)
    }
    /// This function makes the machin's move
    fn machine_move(&mut self) -> Option<Move> {
        if let Some(m) = self.find_best_move() {
            self.do_move(&m, self.machine_char);

            return Some(m)
        }
        
        None
    }

    fn do_move(&mut self, m: &Move, c: BoardChar) -> bool {
        if self.board[m.row][m.col] == BoardChar::Empty {
            self.board[m.row][m.col] = c;

            return true
        }
        
        false
    }
    /// This function returns true if player won
    fn player_won(&self) -> bool {
        self.evaluate(self.player_char)
    }
    /// This function returns true if machine won
    fn machine_won(&self) -> bool {
        self.evaluate(self.machine_char)
    }
    /// This function will return the best possible move for machine
    fn find_best_move(&mut self) -> Option<Move> {
        let mut best_val = -10;
        let mut best_move = None;

        // Traverse all cells, evaluate minimax function for all empty cells.
        // And return the cell with optimal value.
        for i in 0..3 {
            for j in 0..3 {
                // Check if cell is empty
                if self.board[i][j] == BoardChar::Empty {
                    // Make the move
                    self.board[i][j] = self.machine_char;
                    // compute evaluation function for this move.
                    let move_val = self.minimax(self.player_char);
                    // If the move_value is more than the best_val, then update best_val
                    if move_val > best_val {
                        best_move = Some(Move { row: i, col: j });
                        best_val = move_val;
                    }

                    // undo the move
                    self.board[i][j] = BoardChar::Empty;
                }
            }
        }

        best_move
    }

    fn evaluate(&self, c: BoardChar) -> bool {
        // Checking for Rows for X or O victory.
        for row in 0..3 {
            if self.board[row][0] == c && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2] {
                return true;
            }
        }
    
        // Checking for Columns for X or O victory.
        for col in 0..3 {
            if self.board[0][col] == c && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                return true;
            }
        }
    
        // Checking for Diagonals for X or O victory.
        if self.board[0][0] == c && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            return true;
        }
    
        if self.board[0][2] == c && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            return true;
        }
    
        // Else if none of them have won
        false
    }

    // This is the minimax function. It considers all the possible ways
    // the game can go and returns the value of the board
    fn minimax(&mut self, c: BoardChar) -> i16 {
        // If Machine has won the game return his/her evaluated score
        if self.machine_won() {
            return 1;
        }

        // If Player has won the game return his/her evaluated score
        if self.player_won() {
            return -1;
        }

        if !self.has_moves() {
            return 0;
        }

        let mut best: i16 = if c == self.machine_char {
            // If this maximizer's move
            -10
        } else {
            // If this minimizer's move
            10
        };

        for i in 0..3 {
            for j in 0..3 {
                // check if cell is empty
                if self.board[i][j] == BoardChar::Empty {
                    // make the move
                    self.board[i][j] = c;

                    // call minimax recursively
                    let next_best = self.minimax(c.to_opposite());

                    if c == self.machine_char {
                        // choose the maximum value
                        best = cmp::max(best, next_best);
                    } else {
                        // choose the minimum value
                        best = cmp::min(best, next_best);
                    }

                    // undo the move
                    self.board[i][j] = BoardChar::Empty;
                }
            }
        }

        best
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_txt = format!(
            "  A B C\n \u{250C}\u{2500}\u{252C}\u{2500}\u{252C}\u{2500}\u{2510}\n{}\u{2502}",
            1
        );

        for row in &self.board[0] {
            board_txt.push_str(&format!("{}\u{2502}", row));
        }

        board_txt.push_str(&format!(
            "\n \u{251C}\u{2500}\u{253C}\u{2500}\u{253C}\u{2500}\u{2524}\n{}\u{2502}",
            2
        ));

        for row in &self.board[1] {
            board_txt.push_str(&format!("{}\u{2502}", row));
        }

        board_txt.push_str(&format!(
            "\n \u{251C}\u{2500}\u{253C}\u{2500}\u{253C}\u{2500}\u{2524}\n{}\u{2502}",
            3
        ));

        for row in &self.board[2] {
            board_txt.push_str(&format!("{}\u{2502}", row));
        }

        writeln!(
            f,
            "{}\n \u{2514}\u{2500}\u{2534}\u{2500}\u{2534}\u{2500}\u{2518}\n",
            board_txt
        )
    }
}

/// This is a generic function to convert terminal input in some type
fn read_input<T: FromStr<Err = String>>(ask: &str) -> T {
    println!("{}", ask);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match T::from_str(&input) {
            Ok(bc) => return bc,
            Err(e) => {
                println!("{}", e);
                return read_input::<T>(ask);
            }
        },

        Err(error) => panic!(error),
    }
}

fn main() {
    let bc = read_input("Please choose a symbol: X or O");
    let mut game = TicTacToe::new(bc);
    
    while !game.is_game_over() {
        println!("{}", game);

        let m = read_input("your turn: ");
        if !game.player_move(&m) {
            println!("This move is not possible, the cell is already occupied");
            continue;
        }

        if let Some(m) = game.machine_move() {
            println!("machine moved to: {}", m);
        }
    }

    println!("{}", game);
    
    if game.player_won() {
        println!("Congratulations, you won!");
    } else if game.machine_won() {
        println!("Sorry, but you lost");
    } else {
        println!("Draw");
    }
}
