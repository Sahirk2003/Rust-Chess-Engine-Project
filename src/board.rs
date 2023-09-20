struct Board {
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

enum Color {
    White,
    Black
}

struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}