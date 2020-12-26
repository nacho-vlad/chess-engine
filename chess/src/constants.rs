use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {


}


fn generate_slide_horizontal() -> [HashMap<u8, Bitboard>; 64] {
    let mut memo = [HashMap::new(); 64];
    
    for sq in Square::iter() {
        let mut hashmap = HashMap::new();
        let file = sq.file();
        let mut attack = 0u8;
        for pos in 0..256u8 {
            for i in (0..file).rev() {
                attack |= (1<<i);
                if (pos & (1<<i)) != 0 {
                    break;
                }
            }
            for i in file+1..8 {
                attack |= (1<<i);
                if (pos & (1<<i)) != 0 {
                    break;
                }
            }
            let bitb = Bitboard((attack as u64) << (8u64 * sq.rank()));
            memo[sq as usize][pos] = bitb;
        }
    }

    memo
}
