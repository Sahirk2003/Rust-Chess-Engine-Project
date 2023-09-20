#[derive(Debug, Clone)]
pub struct Board {
    // Piece Bitboards
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queen: u64,
    white_king: u64,

    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queen: u64,
    black_king: u64,

    // Game State Trackers
    side_to_move: Color, // Tracking which side's turn it is
    en_passant_target: Option <u8>, // Tracking squares where en passant is possible
    castling_rights: CastlingRights, // Tracking castling rights
    half_move_clock: u32, // Tracking fifty-move rule
    game_move_counter: u32, // Game move counter
}

impl Board {
    pub fn init_board() -> Board {
        Board {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queen: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queen: 0,
            black_king: 0,
            side_to_move: Color::White,
            en_passant_target: None,
            castling_rights: CastlingRights { 
                white_kingside: false, 
                white_queenside: false, 
                black_kingside: false, 
                black_queenside: false 
            },
            half_move_clock: 0,
            game_move_counter: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Color {
    White,
    Black
}

#[derive(Debug, Clone)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}