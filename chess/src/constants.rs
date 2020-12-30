use lazy_static::lazy_static;
use crate::bitboard::{Square,Bitboard,Direction};
use crate::position::{Colored, Color};
use strum::IntoEnumIterator;
use std::collections::HashMap;

lazy_static! {
    pub static ref SLIDE_HORIZONTAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_horizontal();
    pub static ref SLIDE_VERTICAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_vertical();
    pub static ref SLIDE_MAIN_DIAGONAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_main_diagonal();
    pub static ref SLIDE_SECOND_DIAGONAL: HashMap<(Square, Bitboard), (Bitboard, Bitboard)> = generate_slide_second_diagonal();
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = knight_moves();
    pub static ref KING_MOVES: [Bitboard; 64] = king_moves();
    pub static ref PAWN_ATTACKS: Colored<[Bitboard; 64]> = pawn_attacks();
}


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
