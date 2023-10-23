# cube_sat

extern crate bit_vec;

use bit_vec::BitVec;

fn bitwise_or_bitvecs(bitvecs: &[BitVec]) -> BitVec {
    let mut result = BitVec::from_elem(bitvecs[0].len(), false);

    for bv in bitvecs {
        for (index, bit) in bv.iter().enumerate() {
            result.set(index, result[index] | bit);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_or_bitvecs() {
        let bitvec1 = BitVec::from_bytes(&[0b1010]);
        let bitvec2 = BitVec::from_bytes(&[0b1100]);
        let bitvec3 = BitVec::from_bytes(&[0b0110]);

        let bitvecs = vec![bitvec1, bitvec2, bitvec3];

        let result = bitwise_or_bitvecs(&bitvecs);

        let expected_result = BitVec::from_bytes(&[0b1110]);

        assert_eq!(result, expected_result);
    }
}
