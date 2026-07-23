use alloy::primitives::{Address, B256, U256 as AlloyU256};
use ethers::types::{H160, H256, U256 as EthersU256};

pub fn h160_to_address(value: impl AsRef<[u8]>) -> Address {
    Address::from_slice(value.as_ref())
}

pub fn address_to_h160(value: Address) -> H160 {
    H160::from_slice(value.as_slice())
}

pub fn h256_to_b256(value: H256) -> B256 {
    B256::from_slice(value.as_bytes())
}

pub fn b256_to_h256(value: B256) -> H256 {
    H256::from_slice(value.as_slice())
}

pub fn alloy_u256_to_ethers(value: AlloyU256) -> EthersU256 {
    EthersU256::from_big_endian(&value.to_be_bytes::<32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_roundtrips_are_lossless() {
        let address = H160::from_slice(&[0x11; 20]);
        assert_eq!(address_to_h160(h160_to_address(address)), address);

        let hash = H256::from_slice(&[0x22; 32]);
        assert_eq!(b256_to_h256(h256_to_b256(hash)), hash);

        let values = [
            (AlloyU256::from(0_u8), EthersU256::zero()),
            (AlloyU256::from(1_u8), EthersU256::one()),
            (AlloyU256::from(u128::MAX), EthersU256::from(u128::MAX)),
            (AlloyU256::MAX, EthersU256::MAX),
        ];
        for (alloy, ethers) in values {
            assert_eq!(alloy_u256_to_ethers(alloy), ethers);
        }
    }
}
