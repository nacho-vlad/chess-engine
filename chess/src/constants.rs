use lazy_static::lazy_static;
use crate::bitboard::{Square,Bitboard,Direction};
use crate::types::*;
use strum::IntoEnumIterator;
use std::collections::HashMap;

lazy_static! {
    pub static ref SLIDE_HORIZONTAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_horizontal();
    pub static ref SLIDE_VERTICAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_vertical();
    pub static ref SLIDE_MAIN_DIAGONAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_main_diagonal();
    pub static ref SLIDE_SECOND_DIAGONAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_second_diagonal();
    pub static ref KNIGHT_ATTACKS: [Bitboard; 64] = knight_moves();
    pub static ref KING_ATTACKS: [Bitboard; 64] = king_moves();
    pub static ref PAWN_ATTACKS: Colored<[Bitboard; 64]> = pawn_attacks();

    pub static ref FILES_BETWEEN: [[Bitboard; 8]; 8] = generate_files();
    pub static ref RANKS_BETWEEN: [[Bitboard; 8]; 8] = generate_ranks();
    pub static ref MAIN_DIAGS_BETWEEN: [[Bitboard; 15]; 15] = generate_main_diags();
    pub static ref SECOND_DIAGS_BETWEEN: [[Bitboard; 15]; 15] = generate_second_diags();
}

fn generate_files() ->  [[Bitboard; 8]; 8] {
    let mut files = [[Bitboard(0); 8]; 8]; 
    for i in 0..8 {
        for j in 0..8 {
            for k in std::cmp::min(i,j)..std::cmp::max(i,j)+1 {
                files[i][j] = files[i][j] | FILE[k];
            }
        }
    }

    files
}

fn generate_ranks() ->  [[Bitboard; 8]; 8] {
    let mut ranks = [[Bitboard(0); 8]; 8]; 
    for i in 0..8 {
        for j in 0..8 {
            for k in std::cmp::min(i,j)..std::cmp::max(i,j)+1 {
                ranks[i][j] = ranks[i][j] | RANK[k];
            }
        }
    }

    ranks
}

fn generate_main_diags() ->  [[Bitboard; 15]; 15] {
    let mut main_diags = [[Bitboard(0); 15]; 15]; 
    for i in 0..15 {
        for j in 0..15 {
            for k in std::cmp::min(i,j)..std::cmp::max(i,j)+1 {
                main_diags[i][j] = main_diags[i][j] | MAIN_DIAG[k];
            }
        }
    }

    main_diags
}

fn generate_second_diags() ->  [[Bitboard; 15]; 15] {
    let mut second_diags = [[Bitboard(0); 15]; 15]; 
    for i in 0..15 {
        for j in 0..15 {
            for k in std::cmp::min(i,j)..std::cmp::max(i,j)+1 {
                second_diags[i][j] = second_diags[i][j] | SECOND_DIAG[k];
            }
        }
    }

    second_diags
}


pub const SAFE_KING_CASTLE: Colored<Bitboard> = Colored(Bitboard(0x00_00_00_00_00_00_00_70),
                                                        Bitboard(0x70_00_00_00_00_00_00_00));
pub const SAFE_QUEEN_CASTLE: Colored<Bitboard> = Colored(Bitboard(0x00_00_00_00_00_00_00_1C),
                                                        Bitboard(0x1C_00_00_00_00_00_00_00));
pub const FREE_KING_CASTLE: Colored<Bitboard> = Colored(Bitboard(0x00_00_00_00_00_00_00_60),
                                                        Bitboard(0x60_00_00_00_00_00_00_00));
pub const FREE_QUEEN_CASTLE: Colored<Bitboard> = Colored(Bitboard(0x00_00_00_00_00_00_00_0E),
                                                        Bitboard(0x0E_00_00_00_00_00_00_00));

pub const FILE: [Bitboard; 8] = [
    Bitboard(0x0101010101010101),
    Bitboard(0x0202020202020202),
    Bitboard(0x0404040404040404),
    Bitboard(0x0808080808080808),
    Bitboard(0x1010101010101010),
    Bitboard(0x2020202020202020),
    Bitboard(0x4040404040404040),
    Bitboard(0x8080808080808080),
];

pub const RANK: [Bitboard; 8] = [
    Bitboard(0x00000000000000FF),
    Bitboard(0x000000000000FF00),
    Bitboard(0x0000000000FF0000),
    Bitboard(0x00000000FF000000),
    Bitboard(0x000000FF00000000),
    Bitboard(0x0000FF0000000000),
    Bitboard(0x00FF000000000000),
    Bitboard(0xFF00000000000000),
];

pub const MAIN_DIAG: [Bitboard; 15] = [
    Bitboard(0x0000000000000001),
    Bitboard(0x0000000000000102),
    Bitboard(0x0000000000010204),
    Bitboard(0x0000000001020408),
    Bitboard(0x0000000102040810),
    Bitboard(0x0000010204081020),
    Bitboard(0x0001020408102040),
    Bitboard(0x0102040810204080),
    Bitboard(0x0204081020408000),
    Bitboard(0x0408102040800000),
    Bitboard(0x0810204080000000),
    Bitboard(0x1020408000000000),
    Bitboard(0x2040800000000000),
    Bitboard(0x4080000000000000),
    Bitboard(0x8000000000000000),
];

pub const SECOND_DIAG: [Bitboard; 15] = [
    Bitboard(0x01_00_00_00_00_00_00_00),
    Bitboard(0x02_01_00_00_00_00_00_00),
    Bitboard(0x04_02_01_00_00_00_00_00),
    Bitboard(0x08_04_02_01_00_00_00_00),
    Bitboard(0x10_08_04_02_01_00_00_00),
    Bitboard(0x20_10_08_04_02_01_00_00),
    Bitboard(0x40_20_10_08_04_02_01_00),
    Bitboard(0x80_40_20_10_08_04_02_01),
    Bitboard(0x00_80_40_20_10_08_04_02),
    Bitboard(0x00_00_80_40_20_10_08_04),
    Bitboard(0x00_00_00_80_40_20_10_08),
    Bitboard(0x00_00_00_00_80_40_20_10),
    Bitboard(0x00_00_00_00_00_80_40_20),
    Bitboard(0x00_00_00_00_00_00_80_40),
    Bitboard(0x00_00_00_00_00_00_00_80),
];




fn generate_slide_horizontal() -> HashMap<(Square, Bitboard), (Bitboard, Bitboard)>{
    let mut memo = HashMap::new();

    for sq in Square::iter() {    
        for pos in 0..=255u8 {
            let mut key = 0u64;
            let mut attack = 0u64;
            let mut pinned = 0u64;

            let mut attack_flag = true;
            let mut pinned_flag = false;

            for left in sq.ray(Direction::Left) {
                let rank = left.file();
                
                if attack_flag {
                    attack |= left.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= left.to_bitboard().0;
                }
                if (1<<rank)&pos != 0{
                    key |= left.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false;
                }
            }
            attack_flag = true;
            pinned_flag = false;
            for right in sq.ray(Direction::Right) {        
                let rank = right.file();
                
                if attack_flag {
                    attack |= right.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= right.to_bitboard().0;
                }
                if (1<<rank)&pos != 0 {
                    key |= right.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false
                }
            }
            let attack = Bitboard(attack);
            let pinned = Bitboard(pinned);
            memo.insert((sq,Bitboard(key)), (attack, pinned));
            key |= sq.to_bitboard().0;
            memo.insert((sq,Bitboard(key)), (attack, pinned));
        }
    }
    memo
}


fn generate_slide_vertical() -> HashMap<(Square,Bitboard), (Bitboard, Bitboard)>  {
    let mut memo = HashMap::new();

    for sq in Square::iter() {    
        for pos in 0..=255u8 {
            let mut key = 0u64;
            let mut attack = 0u64;
            let mut pinned = 0u64;

            let mut attack_flag = true;
            let mut pinned_flag = false;

            for above in sq.ray(Direction::Up) {
                let rank = above.rank();
                
                if attack_flag {
                    attack |= above.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= above.to_bitboard().0;
                }
                if (1<<rank)&pos != 0{
                    key |= above.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false;
                }
            }
            attack_flag = true;
            pinned_flag = false;
            for below in sq.ray(Direction::Down) {        
                let rank = below.rank();
                
                if attack_flag {
                    attack |= below.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= below.to_bitboard().0;
                }
                if (1<<rank)&pos != 0 {
                    key |= below.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false
                }
            }
            let attack = Bitboard(attack);
            let pinned = Bitboard(pinned);
            memo.insert((sq,Bitboard(key)), (attack, pinned));
            key |= sq.to_bitboard().0;
            memo.insert((sq,Bitboard(key)), (attack, pinned));
        }
    }
    memo
}

fn generate_slide_main_diagonal() -> HashMap<(Square,Bitboard), (Bitboard, Bitboard)> {
    let mut memo = HashMap::new();

    for sq in Square::iter() {    
        for pos in 0..=255u8 {
            let mut key = 0u64;
            let mut attack = 0u64;
            let mut pinned = 0u64;

            let mut attack_flag = true;
            let mut pinned_flag = false;

            for above in sq.ray(Direction::UpLeft) {
                let rank = above.rank();
                
                if attack_flag {
                    attack |= above.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= above.to_bitboard().0;
                }
                if (1<<rank)&pos != 0{
                    key |= above.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false;
                }
            }
            attack_flag = true;
            pinned_flag = false;
            for below in sq.ray(Direction::DownRight) {        
                let rank = below.rank();
                
                if attack_flag {
                    attack |= below.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= below.to_bitboard().0;
                }
                if (1<<rank)&pos != 0 {
                    key |= below.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false
                }
            }
            let attack = Bitboard(attack);
            let pinned = Bitboard(pinned);
            memo.insert((sq,Bitboard(key)), (attack, pinned));
            key |= sq.to_bitboard().0;
            memo.insert((sq,Bitboard(key)), (attack, pinned));
        }
    }
    memo

}


fn generate_slide_second_diagonal() -> HashMap<(Square,Bitboard), (Bitboard, Bitboard)> {
    let mut memo = HashMap::new();

    for sq in Square::iter() {    
        for pos in 0..=255u8 {
            let mut key = 0u64;
            let mut attack = 0u64;
            let mut pinned = 0u64;

            let mut attack_flag = true;
            let mut pinned_flag = false;

            for above in sq.ray(Direction::UpRight) {
                let rank = above.rank();
                
                if attack_flag {
                    attack |= above.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= above.to_bitboard().0;
                }
                if (1<<rank)&pos != 0{
                    key |= above.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false;
                }
            }
            attack_flag = true;
            pinned_flag = false;
            for below in sq.ray(Direction::DownLeft) {        
                let rank = below.rank();
                
                if attack_flag {
                    attack |= below.to_bitboard().0;
                }
                if pinned_flag {
                    pinned |= below.to_bitboard().0;
                }
                if (1<<rank)&pos != 0 {
                    key |= below.to_bitboard().0;
                    pinned_flag = if attack_flag { true } else { false };
                    attack_flag = false
                }
            }
            let attack = Bitboard(attack);
            let pinned = Bitboard(pinned);
            memo.insert((sq,Bitboard(key)), (attack, pinned));
            key |= sq.to_bitboard().0;
            memo.insert((sq,Bitboard(key)), (attack, pinned));
        }
    }
    memo

}


fn knight_moves() -> [Bitboard; 64] {
    let mut attacks = [Bitboard(0); 64]; 
    
    for sq in 0..63i8 {
        for dp in [6,15,17,10,-6,-15,-17,-10i8].iter() {
            if sq + dp < 0 || sq + dp > 63 {
                continue;
            }
            if ((sq+dp)%8 - sq%8).abs() > 2 {
                continue;
            }
            let attack = Square::from((sq+dp) as u8);
            attacks[sq as usize].0 |= attack.to_bitboard().0;
        }
    }
    attacks
}

fn king_moves() -> [Bitboard; 64] {
    let mut attacks = [Bitboard(0); 64]; 
    
    for sq in 0..63i8 {
        for dp in [7,8,9,1,-7,-8,-9,-1].iter() {
            if sq + dp < 0 || sq + dp > 63 {
                continue;
            }
            if ((sq+dp)%8 - sq%8).abs() > 2 {
                continue;
            }
            let attack = Square::from((sq+dp) as u8);
            attacks[sq as usize].0 |= attack.to_bitboard().0;
        }
    }
    attacks
}

fn pawn_attacks() -> Colored<[Bitboard; 64]> {
    let mut attacks = Colored([Bitboard(0);64], [Bitboard(0);64]);
    
    for sq in 0..63i8 {
        for dp in [7,9, -9, -7].iter() {
            if sq + dp < 0 || sq + dp > 63 {
                continue;
            }
            if ((sq+dp)%8 - sq%8).abs() > 2 {
                continue;
            }
            let attack = Square::from((sq+dp) as u8);
            let color = if *dp > 0 { Color::White } else { Color::Black };
            attacks[color][sq as usize].0 |= attack.to_bitboard().0;
        }
    }
    
    attacks
}
