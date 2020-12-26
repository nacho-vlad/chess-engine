use lazy_static::lazy_static;
use crate::bitboard::{Square,Bitboard,Direction};
use strum::IntoEnumIterator;
use std::collections::HashMap;

lazy_static! {
    pub static ref SLIDE_HORIZONTAL: Vec<HashMap<u8, Bitboard>> = generate_slide_horizontal();
    pub static ref SLIDE_VERTICAL: HashMap<(Square, Bitboard), Bitboard> = generate_slide_vertical();
}


fn generate_slide_horizontal() -> Vec<HashMap<u8, Bitboard>>{
    let mut memo: Vec<HashMap<u8, Bitboard>> = vec![HashMap::new(); 64];
    
    for sq in Square::iter() {
        let file = sq.file();
        for pos in 0..=255u8 {
            let mut attack = 0u8;
            for i in (0..file).rev() {
                attack |= 1<<i;
                if (pos & (1<<i)) != 0 {
                    break;
                }
            }
            for i in file+1..8 {
                attack |= 1<<i;
                if (pos & (1<<i)) != 0 {
                    break;
                }
            }
            let bitb = Bitboard((attack as u64) << (8u64 * sq.rank() as u64));
            memo[sq as usize].insert(pos, bitb);
        }
    }

    memo
}


fn generate_slide_vertical() -> HashMap<(Square,Bitboard), Bitboard>  {
    let mut memo: HashMap<(Square,Bitboard), Bitboard> = HashMap::new();

    for sq in Square::iter() {    
        for pos in 0..=255u8 {
            let mut key = 0u64;
            let mut attack = 0u64;

            let mut flag = true;
            for above in sq.ray(Direction::Up) {
                let rank = above.rank();
                
                if flag {
                    attack |= above.to_bitboard().0;
                }
                if (1<<rank)&pos != 0{
                    key |= above.to_bitboard().0;
                    flag = false
                }
            }
            flag = true;
            for below in sq.ray(Direction::Down) {        
                let rank = below.rank();
                
                if flag {
                    attack |= below.to_bitboard().0;
                }
                if (1<<rank)&pos != 0 {
                    key |= below.to_bitboard().0;
                    flag = false
                }
            }
            memo.insert((sq,Bitboard(key)), Bitboard(attack));
            key |= sq.to_bitboard().0;
            memo.insert((sq,Bitboard(key)), Bitboard(attack));
        }
    }
    memo
}



