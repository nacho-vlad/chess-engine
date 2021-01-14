use std::hash::Hash;
use std::collections::HashMap;
use std::str::FromStr;

use crate::constants::*;
use crate::repr::*;
use crate::ChessError;


type Board = Colored<Pieces>;


#[derive(Copy, Clone, Debug, Hash, Eq)]
pub struct Position {
    pub board: Board, 
    pub turn: Color,
    pub castling_rights: Colored<CastlingRights>,
    pub en_passant: Option<Square>,
    pub half_moves: u16,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board &&
        self.turn == other.turn &&
        self.castling_rights == other.castling_rights &&
        self.en_passant == other.en_passant
    }
}

fn intersects(lhs: Bitboard, rhs: Bitboard) -> bool {
    lhs & rhs != Bitboard::empty()
}

fn between(lhs: u8, between: u8, rhs: u8) -> bool {
    std::cmp::min(lhs, rhs) < between && between < std::cmp::max(lhs,rhs)
}


fn bishop_attack(occupied: Bitboard, square: Square) -> Bitboard {
    let main_diag = MAIN_DIAG[square.main_diag() as usize];
    let main_attacks = SLIDE_MAIN_DIAGONAL[&(square, main_diag & occupied)];
    let second_diag = SECOND_DIAG[square.second_diag() as usize];
    let second_attacks = SLIDE_SECOND_DIAGONAL[&(square, second_diag & occupied)]; 

    main_attacks.0 | second_attacks.0        
}

fn rook_attack(occupied: Bitboard, square: Square) -> Bitboard {
    let rank = RANK[square.rank() as usize];
    let horizontal_attacks = SLIDE_HORIZONTAL[&(square, rank & occupied)];
    let file = FILE[square.file() as usize];
    let vertical_attacks = SLIDE_VERTICAL[&(square, file & occupied)]; 

    horizontal_attacks.0 | vertical_attacks.0      
}

fn queen_attack(occupied: Bitboard, square: Square) -> Bitboard {
    let bishop_attacks = bishop_attack(occupied, square);
    let rook_attacks = rook_attack(occupied, square);

    bishop_attacks | rook_attacks
}

fn knight_attack(square: Square) -> Bitboard {
    KNIGHT_ATTACKS[square as usize]
}

fn king_attack(square: Square) -> Bitboard {
    KING_ATTACKS[square as usize]
}

fn pawn_attack(color: Color, square: Square) -> Bitboard {
    PAWN_ATTACKS[color][square as usize]
}



impl Board {
    pub fn occupied(&self) -> Bitboard {
        self[Color::White].occupied() | self[Color::Black].occupied()
    }
    
    pub fn unoccupied(&self) -> Bitboard { 
        self[Color::White].unoccupied() | self[Color::Black].unoccupied()
    }
    
    pub fn attacks(&self, color: Color) -> Bitboard {
        let occupied = self.occupied();
        let mut attacked = Bitboard::empty();

        for knight in self[color][Piece::Knight].squares() {
            attacked = attacked | knight_attack(knight);
        }
        for bishop in self[color][Piece::Bishop].squares() {
            attacked = attacked | bishop_attack(occupied, bishop);
        }
        for rook in self[color][Piece::Rook].squares() {
            attacked = attacked | rook_attack(occupied, rook);
        }
        for queen in self[color][Piece::Queen].squares() {
            attacked = attacked | queen_attack(occupied, queen);
        }
        for pawn in self[color][Piece::Pawn].squares() {
            attacked = attacked | pawn_attack(color, pawn);
        }
        attacked
    }

    pub fn in_check(&self, player: Color) -> bool {
        let opponent = player.other();
        let occupied = self.occupied();
        let mut attacked = Bitboard::empty();

        for knight in self[opponent][Piece::Knight].squares() {
            attacked = attacked | knight_attack(knight);
        }
        for bishop in self[opponent][Piece::Bishop].squares() {
            attacked = attacked | bishop_attack(occupied, bishop);
        }
        for rook in self[opponent][Piece::Rook].squares() {
            attacked = attacked | rook_attack(occupied, rook);
        }
        for queen in self[opponent][Piece::Queen].squares() {
            attacked = attacked | queen_attack(occupied, queen);
        }
        for pawn in self[opponent][Piece::Pawn].squares() {
            attacked = attacked | pawn_attack(opponent, pawn);
        }

        intersects(self[player][Piece::King], attacked)
    }

    pub fn at(&self, square: Square) -> Option<(Color,Piece)> {
        let white = self[Color::White].at(square);
        let black = self[Color::Black].at(square);
        match (white, black) {
            (None, None) => None,
            (Some(piece), None) => Some((Color::White,piece)),
            (None, Some(piece)) => Some((Color::Black,piece)),
            (Some(_), Some(_)) => panic!("2 pieces on the same square"),
        }
    }

}

impl Position {

    pub fn in_check(&self) -> bool {
        return self.board.in_check(self.turn)
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        
        let pieces = self.board;
        let player = self.turn;
        let opponent = player.other();
        let mut attacked = Bitboard::empty();
        
        let mut pinned: HashMap<Square, Bitboard> = HashMap::new();
        let mut nr_checks: u8 = 0;
        let mut protect_king = Bitboard::full();

        let mut all_occupied = pieces.occupied();
        let player_occupied = pieces[player].occupied();

        let king = pieces[player][Piece::King];
        let king_sq = match king.squares().next() {
            None => return legal_moves,
            Some(sq) => sq,
        };

        all_occupied.unset(king_sq);
        //Opponent pawns
        for sq in pieces[opponent][Piece::Pawn].squares() {
            let attacks = PAWN_ATTACKS[opponent][sq as usize];
            if intersects(attacks, king) {
                nr_checks += 1;
                protect_king = sq.to_bitboard();
            }
            attacked = attacked | attacks;
        }

        //Opponent knights
        for sq in pieces[opponent][Piece::Knight].squares() {
            let attacks = KNIGHT_ATTACKS[sq as usize];
            if intersects(attacks, king) {
                nr_checks += 1;
                protect_king = sq.to_bitboard();
            }
            attacked = attacked | attacks;
        }

        //Opponent rooks and queen;
        for rook in pieces[opponent][Piece::Rook].squares().chain(pieces[opponent][Piece::Queen].squares()) {
            let (h_attack, h_pin) = SLIDE_HORIZONTAL[&(rook, all_occupied & RANK[rook.rank() as usize])];

            if intersects(h_attack,king) {
                nr_checks += 1;
                protect_king = h_attack & FILES_BETWEEN[rook.file() as usize][king_sq.file() as usize] | rook.to_bitboard(); 
            }

            if intersects(h_pin, king) {
                for sq in (player_occupied & h_attack).squares() {
                    if between(king_sq.file(), sq.file(), rook.file()) {
                        pinned.insert(sq, RANK[sq.rank() as usize]);
                    }
                }
            }

            let (v_attack, v_pin) = SLIDE_VERTICAL[&(rook, all_occupied & FILE[rook.file() as usize])];

            if intersects(v_attack,king) {
                nr_checks += 1;
                protect_king = v_attack & RANKS_BETWEEN[rook.rank() as usize][king_sq.rank() as usize] | rook.to_bitboard(); 
            }

            if intersects(v_pin, king) {
                let king_sq = king.squares().next().unwrap();
                for sq in (player_occupied & v_attack).squares() {
                    if between(king_sq.rank(), sq.rank(), rook.rank()) {
                        pinned.insert(sq, FILE[sq.file() as usize]);
                    }
                }
            }

            attacked = attacked | h_attack | v_attack;
        }
        
        //Opponent bishops and queen
        for bishop in pieces[opponent][Piece::Bishop].squares().chain(pieces[opponent][Piece::Queen].squares()) {
            let (md_attack, md_pin) = SLIDE_MAIN_DIAGONAL[&(bishop, all_occupied & MAIN_DIAG[bishop.main_diag() as usize])];

            if intersects(md_attack,king) {
                nr_checks += 1;
                protect_king = md_attack & SECOND_DIAGS_BETWEEN[bishop.second_diag() as usize][king_sq.second_diag() as usize] | bishop.to_bitboard(); 
            }

            if intersects(md_pin, king) {
                let king_sq = king.squares().next().unwrap();
                for sq in (player_occupied & md_attack).squares() {
                    if between(king_sq.second_diag(), sq.second_diag(), bishop.second_diag()) {
                        pinned.insert(sq, MAIN_DIAG[sq.main_diag() as usize]);
                    }
                }
            }

            let (sd_attack, sd_pin) = SLIDE_SECOND_DIAGONAL[&(bishop, all_occupied & SECOND_DIAG[bishop.second_diag() as usize])];

            if intersects(sd_attack,king) {
                nr_checks += 1;
                protect_king = sd_attack & MAIN_DIAGS_BETWEEN[bishop.main_diag() as usize][king_sq.main_diag() as usize] | bishop.to_bitboard(); 
            }

            if intersects(sd_pin, king) {
                let king_sq = king.squares().next().unwrap();
                for sq in (player_occupied & sd_attack).squares() {
                    if between(king_sq.main_diag(), sq.main_diag(),  bishop.main_diag())  {
                        pinned.insert(sq, SECOND_DIAG[sq.second_diag() as usize]);
                    }
                }
            }

            attacked = attacked | md_attack | sd_attack;
        }

        //Opponent king {
        for sq in pieces[opponent][Piece::King].squares() {
            let attacks = king_attack(sq);
            attacked = attacked | attacks;
        }
        
        all_occupied.set(king_sq);

        //King moves
        for king_sq in king.squares() {
            let move_sqs = king_attack(king_sq);
            for dest in (move_sqs & !attacked & !player_occupied).squares() {
                legal_moves.push(Move::Normal(king_sq,dest, Piece::King));
            }
        }

        if nr_checks > 1 {
            return legal_moves
        }

        //Knight moves
        for knight in pieces[player][Piece::Knight].squares() {
            let jumps = knight_attack(knight);
            let legal_squares = match pinned.get(&knight) {
                None => jumps & protect_king & !player_occupied,
                Some(&pin) => jumps & protect_king & !player_occupied & pin,
            };
            for dest in legal_squares.squares() {
                legal_moves.push(Move::Normal(knight, dest, Piece::Knight));
            }
        }

        //Bishop moves
        for bishop in pieces[player][Piece::Bishop].squares() {
            let slides = bishop_attack(all_occupied, bishop);
            let legal_squares = match pinned.get(&bishop) {
                None => slides & protect_king & !player_occupied,
                Some(&pin) => slides & protect_king & !player_occupied & pin,
            };
            for dest in legal_squares.squares() {
                legal_moves.push(Move::Normal(bishop, dest, Piece::Bishop));
            }
        }

        //Rook moves
        for rook in pieces[player][Piece::Rook].squares() {
            let slides = rook_attack(all_occupied, rook);
            let legal_squares = match pinned.get(&rook) {
                None => slides & protect_king & !player_occupied,
                Some(&pin) => slides & protect_king & !player_occupied & pin,
            };
            for dest in legal_squares.squares() {
                legal_moves.push(Move::Normal(rook, dest, Piece::Rook));
            }
        }

        //Queen moves
        for queen in pieces[player][Piece::Queen].squares() {
            let slides = queen_attack(all_occupied, queen);
            let legal_squares = match pinned.get(&queen) {
                None => slides & protect_king & !player_occupied,
                Some(&pin) => slides & protect_king & !player_occupied & pin,
            };
            for dest in legal_squares.squares() {
                legal_moves.push(Move::Normal(queen, dest, Piece::Queen));
            }
        }
        
        //Pawn pushes
        if player == Color::White {
            let single_pushes = self.board[player][Piece::Pawn].0 << 8;
            let single_pushes = single_pushes & !all_occupied.0;
            let double_pushes = (single_pushes << 8) & RANK[3].0;

            for dest in Bitboard(single_pushes).squares() {
                let pawn = dest + Direction::Down;
                let legal_squares = match pinned.get(&pawn) {
                    None => protect_king & !all_occupied,
                    Some(&pin) => protect_king & !all_occupied & pin,
                };
                if !intersects(legal_squares, dest.to_bitboard()) {
                    continue;
                }
                if dest.rank() == 7 {
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Queen));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Rook));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Knight));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Bishop));
                } else {
                    legal_moves.push(Move::Normal(pawn,dest, Piece::Pawn));
                }
            }

            for dest in Bitboard(double_pushes).squares() {
                let pawn = dest + Direction::Down + Direction::Down;
                let legal_squares = match pinned.get(&pawn) {
                    None => protect_king & !all_occupied,
                    Some(&pin) => protect_king & !all_occupied & pin,
                };
                if !intersects(legal_squares, dest.to_bitboard()) {
                    continue;
                }
                legal_moves.push(Move::Normal(pawn,dest, Piece::Pawn));
            }
            
        } else {
            let single_pushes = self.board[player][Piece::Pawn].0 >> 8;
            let single_pushes = single_pushes & !all_occupied.0;
            let double_pushes = (single_pushes >> 8) & RANK[4].0;

            for dest in Bitboard(single_pushes).squares() {
                let pawn = dest + Direction::Up;
                let legal_squares = match pinned.get(&pawn) {
                    None => protect_king & !all_occupied,
                    Some(&pin) => protect_king & !all_occupied & pin,
                };
                if !intersects(legal_squares, dest.to_bitboard()) {
                    continue;
                }
                if dest.rank() == 0 {
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Queen));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Rook));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Knight));
                    legal_moves.push(Move::Promotion(pawn,dest, Piece::Bishop));
                } else {
                    legal_moves.push(Move::Normal(pawn,dest, Piece::Pawn));
                }
            }

            for dest in Bitboard(double_pushes).squares() {
                let pawn = dest + Direction::Up + Direction::Up;
                let legal_squares = match pinned.get(&pawn) {
                    None => protect_king & !all_occupied,
                    Some(&pin) => protect_king & !all_occupied & pin,
                };
                if !intersects(legal_squares, dest.to_bitboard()) {
                    continue;
                }
                legal_moves.push(Move::Normal(pawn,dest, Piece::Pawn));
            }

        }
    
        let opponent_pieces = pieces[opponent].occupied();
        //Pawn attacks
        for pawn in self.board[player][Piece::Pawn].squares() {
            let attack = pawn_attack(player, pawn); 
            let en_passant_bb = match self.en_passant {
                None => Bitboard::empty(),
                Some(sq) => sq.to_bitboard(),
            };
            let legal_squares = match pinned.get(&pawn) {
                None => protect_king & opponent_pieces | en_passant_bb,
                Some(&pin) => protect_king & opponent_pieces & pin | en_passant_bb,
            };

            for dest in (attack & legal_squares).squares() {
                if let Some(ep) = self.en_passant {
                    if ep == dest {
                        let mut board = pieces;
                        board[player][Piece::Pawn].unset(pawn);
                        board[player][Piece::Pawn].set(dest);
                        board[opponent][Piece::Pawn].unset(ep + Direction::pawn(opponent));
                        if !board.in_check(player) {
                            legal_moves.push(Move::EnPassant(pawn,dest));
                        }
                        continue;
                    }
                }
                let promote_rank = match player {
                    Color::White => 7,
                    Color::Black => 0,
                };
                if dest.rank() == promote_rank { 
                    legal_moves.push(Move::Promotion(pawn,dest,Piece::Queen));
                    legal_moves.push(Move::Promotion(pawn,dest,Piece::Rook));
                    legal_moves.push(Move::Promotion(pawn,dest,Piece::Bishop));
                    legal_moves.push(Move::Promotion(pawn,dest,Piece::Knight));
                } else {
                    legal_moves.push(Move::Normal(pawn,dest,Piece::Pawn));
                }
            }
        }

        //Kingside Castle
        if self.castling_rights[player].kingside && 
           !intersects(SAFE_KING_CASTLE[player], attacked) &&
           !intersects(FREE_KING_CASTLE[player], all_occupied) {
            legal_moves.push(Move::KingsideCastle);
        }

        //Queenside Castle
        if self.castling_rights[player].queenside && 
           !intersects(SAFE_QUEEN_CASTLE[player], attacked) &&
           !intersects(FREE_QUEEN_CASTLE[player], all_occupied) {
            legal_moves.push(Move::QueensideCastle);
        }

        legal_moves
    }

    pub fn make_move(&self, legal_move: Move) -> Position {
        let mut position = *self;
        let player = position.turn;
        let opponent = player.other();
        position.en_passant = None;
        let mut reset_moves = false;

        match legal_move {
            Move::Normal(src,dst,piece) => {
                position.board[player][piece].unset(src);
                position.board[player][piece].set(dst);
                if let Some(piece) = position.board[opponent].at(dst) {
                    position.board[opponent][piece].unset(dst);
                    reset_moves = true;
                }
                if piece == Piece::Rook {
                    if src == Square::A1 || src == Square::A8 {
                        position.castling_rights[player].queenside = false; 
                    }
                    if src == Square::H1 || src == Square::H8 {
                        position.castling_rights[player].kingside = false;
                    }
                }
                if piece == Piece::King {
                    position.castling_rights[player].kingside = false;
                    position.castling_rights[player].queenside = false;
                }
                if piece == Piece::Pawn && (src.rank() as i8 - dst.rank() as i8).abs() > 1 {
                    position.en_passant = Some(src + Direction::pawn(player));
                    reset_moves = true;
                }
            },
            Move::EnPassant(src,dst) => {
                position.board[player][Piece::Pawn].unset(src);
                position.board[player][Piece::Pawn].set(dst);
                position.board[opponent].unset(dst + Direction::pawn(opponent));
                reset_moves = true;
            },
            Move::Promotion(src,dst,to) => {
                position.board[player][Piece::Pawn].unset(src);
                position.board[player][to].set(dst);
                if let Some(piece) = position.board[opponent].at(dst) {
                    position.board[opponent][piece].unset(dst);
                    reset_moves = true;
                }
            },
            Move::KingsideCastle => {
                let (old_king, old_rook, new_king, new_rook) = match player {
                    Color::White => (Square::E1, Square::H1, Square::G1, Square::F1),
                    Color::Black => (Square::E8, Square::H8, Square::G8, Square::F8),
                };
                position.board[player][Piece::King].unset(old_king);
                position.board[player][Piece::King].set(new_king);
                position.board[player][Piece::Rook].unset(old_rook);
                position.board[player][Piece::Rook].set(new_rook);
                
                position.castling_rights[player].kingside = false;
                position.castling_rights[player].queenside = false;
            },
            Move::QueensideCastle => {
                let (old_king, old_rook, new_king, new_rook) = match player {
                    Color::White => (Square::E1, Square::A1, Square::C1, Square::D1),
                    Color::Black => (Square::E8, Square::A8, Square::C8, Square::D8),
                };
                position.board[player][Piece::King].unset(old_king);
                position.board[player][Piece::King].set(new_king);
                position.board[player][Piece::Rook].unset(old_rook);
                position.board[player][Piece::Rook].set(new_rook);

                position.castling_rights[player].kingside = false;
                position.castling_rights[player].queenside = false;
            },
        }
        
        let mut q_r = false;
        let mut k_r = false;
        for sq in position.board[opponent][Piece::Rook].squares() {
            let (queen_rook, king_rook) = match opponent {
                Color::White => (Square::A1, Square::H1),
                Color::Black => (Square::A8, Square::H8),
            };
            if sq == queen_rook {
                q_r = true;
            }
            if sq == king_rook {
                k_r = true;
            }
            
        }
        
        if !q_r {
            position.castling_rights[opponent].queenside = false;
        }
        if !k_r {
            position.castling_rights[opponent].kingside = false;
        }

        position.turn = opponent;
        if reset_moves {
            position.half_moves = 0;
        } else {
            position.half_moves +=1;
        }
        position
    }

    pub fn perft(&self, depth: u32) -> u64 {
        let moves = self.legal_moves();
        let mut nodes = 0;

        if depth == 0 {
            return 1
        }
        if depth == 1 {
            return moves.len() as u64
        }

        for mv in moves.iter() {
            let new_nodes = self.make_move(*mv).perft(depth-1);
            // if depth == 2 {
            //     println!("{}: {}", mv, new_nodes)
            // }
            nodes += new_nodes
        }

        nodes
    }
}

// Input output implementation
impl Position {

    pub fn starting() -> Position {
        Position::from_fen(STARTING_POS_FEN).unwrap()
    }
     
    pub fn from_fen(fen: &str) -> Result<Position, ChessError> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        
        let mut board = Colored(Pieces::empty(), Pieces::empty());
        let mut sq = Square::A8 as i8;
        for c in parts[0].chars() {
            match c {
                'P' => board[Color::White][Piece::Pawn].set(Square::from(sq as u8)),
                'N' => board[Color::White][Piece::Knight].set(Square::from(sq as u8)),
                'B' => board[Color::White][Piece::Bishop].set(Square::from(sq as u8)),
                'R' => board[Color::White][Piece::Rook].set(Square::from(sq as u8)),
                'Q' => board[Color::White][Piece::Queen].set(Square::from(sq as u8)),
                'K' => board[Color::White][Piece::King].set(Square::from(sq as u8)),
                'p' => board[Color::Black][Piece::Pawn].set(Square::from(sq as u8)),
                'n' => board[Color::Black][Piece::Knight].set(Square::from(sq as u8)),
                'b' => board[Color::Black][Piece::Bishop].set(Square::from(sq as u8)),
                'r' => board[Color::Black][Piece::Rook].set(Square::from(sq as u8)),
                'q' => board[Color::Black][Piece::Queen].set(Square::from(sq as u8)),
                'k' => board[Color::Black][Piece::King].set(Square::from(sq as u8)),
                '/' => sq -= 17, 
                d if '1' <= d && d <= '9' => sq += (d as u8 - '0' as u8 - 1) as i8,
                _ => return Err(ChessError::InvalidFEN("Invalid pieces FEN".to_owned()))
            }
            sq += 1;
        }

        let turn = match parts[1] {
            "w" | "W" => Color::White,
            "b" | "B" => Color::Black,
            _ => return Err(ChessError::InvalidFEN("Invalid turn FEN".to_owned())),
        };
        
        let mut white_kingside = false;
        let mut white_queenside = false;
        let mut black_kingside = false;
        let mut black_queenside = false;

        for c in parts[2].chars() {
            match c {
                'K' => white_kingside = true,
                'Q' => white_queenside = true,
                'k' => black_kingside = true,
                'q' => black_queenside = true,
                '-' => {},
                _ => return Err(ChessError::InvalidFEN("Invalid castling rights FEN".to_owned()))
            }
        }

        let castling_rights = Colored(CastlingRights::new(white_kingside, white_queenside),
                                      CastlingRights::new(black_kingside, black_queenside));

        let en_passant = match parts[3] {
            "-" => None,
            sq => Some(Square::from_str(sq).unwrap())
        };

        let half_moves = str::parse::<u16>(parts[4]).unwrap();

        Ok(Position { board, turn, castling_rights, en_passant, half_moves} )
    }


    pub fn to_ascii(&self) -> String {
        use colored::Colorize;
        let mut board: String = String::new();
        
        for row in (0..8u8).rev() {
            for col in 0..8u8 {
                    let white_piece = self.board[Color::White].at(Square::from(row*8+col)); 
                    let black_piece = self.board[Color::Black].at(Square::from(row*8+col)); 
                    let sq = match (white_piece, black_piece) {
                        (None, None) => ".".to_owned(),
                        (Some(piece), None) => piece.to_ascii().to_string().cyan().to_string(),
                        (None, Some(piece)) => piece.to_ascii().to_string().magenta().to_string(),
                        _ => panic!("Impossible to have 2 pieces of different colors on the same square")
                    };
                    board.push_str(&sq);
                    board.push_str(" ");
                }

            board.push_str(&(row+1).to_string());
            board.push('\n');
        }
        board.push_str("a b c d e f g h \n");
        board
    }



}

