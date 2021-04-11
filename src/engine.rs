use core::fmt;

const STARTING_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut board: Board = Board {
        squares: [Space { piece: None }; 64],
    };
    parse_fen_string_to_board(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        &mut board,
    );
}

const VALID_PIECE_CHARS: [char; 6] = ['p', 'n', 'b', 'k', 'q', 'r'];

#[derive(Clone, Copy, Debug)]
enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug)]
enum PieceColor {
    White,
    Black,
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}

#[derive(Clone, Copy)]
struct Space {
    piece: Option<Piece>,
}

struct Board {
    squares: [Space; 64],
}

fn rank_and_file_to_index(rank: u32, file: u32) -> usize {
    (rank * 8 + file) as usize
}

fn parse_fen_string_to_board(fen_string: &str, board: &mut Board) -> () {
    /*  Separate different parts of the fen string. If the string is properly formatted, the parts and
        their matching indices are:
        0: Piece Placement
        1: Side to move ('w' if white's move, 'b' if black's move)
        2: Castling Ability
        3: En passant target square
        4: Halfmove Clock
        5: Fullmove counter - Number of full moves in a game, incremented after each black move.
        For more formatting info, see https://www.chessprogramming.org/Forsyth-Edwards_Notation
    */
    let fen_parts: Vec<&str> = fen_string.split(" ").collect();
    let board_state = fen_parts[0];

    // Reminder for the small-brained (aka JP): Ranks = rows, file = columns
    // NOTE on endianness of FEN string:
    // Ranks are big-endian, so 8 -> 1
    // Files are little-endian, so A -> H
    let mut rank = 7;
    let mut file = 0;
    for c in board_state.chars() {
        if c == '/' {
            file = 0;
            rank -= 1;
        } else {
            if c.is_numeric() {
                let num_to_skip = c.to_digit(10).unwrap();
                file += num_to_skip;
                println!("Skipping {} files.", num_to_skip);
            } else {
                if !VALID_PIECE_CHARS.contains(&c.to_ascii_lowercase()) {
                    // This code shouldn't be reached, but just in case we'll swallow the weird character and move on.
                    // Space should be nothing by default, so just leave it.
                    file += 1;
                } else {
                    let piece_color = if c.is_uppercase() {
                        PieceColor::White
                    } else {
                        PieceColor::Black
                    };
                    let piece_type = match c.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => PieceType::Pawn,
                    };

                    println!(
                        "There is a {} {} in space {},{}",
                        piece_color.to_string(),
                        piece_type.to_string(),
                        rank,
                        file
                    );

                    board.squares[rank_and_file_to_index(rank, file)] = Space {
                        piece: Some(Piece {
                            piece_type,
                            piece_color,
                        }),
                    };
                    file += 1;
                }
            }
        }
    }
}
