use keta_core::block::Block;
use keta_crypto::Hash;
use keta_crypto::Nonce;
use num_bigint::BigInt;

const TARGET_BITS: u64 = 4;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TARGET: BigInt = BigInt::from(1) << 256 - TARGET_BITS;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MineResult {
    pub hash: Hash,
    pub nonce: Nonce,
}

pub fn mine_block(block: &Block) -> MineResult {
    for nonce in 0..Nonce::MAX {
        let hash = block.hash_with_nonce(nonce);
        let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, hash.as_bytes());
        if hash_int.cmp(&TARGET) == std::cmp::Ordering::Less {
            return MineResult { hash, nonce };
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use votate_core::BlockIndex;

    #[test]
    fn mine() {
        let expected_mine_result = MineResult {
            hash: Hash::from_str(
                "0884c68f6de1285c00cb74dc197961bb47c701d924224b4683a13a1066fed186",
            )
            .unwrap(),
            nonce: 1,
        };
        let block = Block {
            index: BlockIndex::from(100),
            timestamp: chrono::MAX_DATETIME,
            prev_hash: Hash::ZERO,
            transactions: vec![],
        };
        let mine_result = mine_block(&block);
        assert_eq!(mine_result, expected_mine_result);
    }
}
