use crate::sha256::Sha256;

const GENERATOR_DOMAIN: &[u8] = b"BitCards/CardGenerator\0";
const STREAM_DOMAIN: &[u8] = b"BitCards/HashStream/v1\0";

/// A deterministic SHA-256 byte stream. This is protocol derivation, not system RNG.
#[derive(Debug, Clone)]
pub struct HashStream {
    key: [u8; 32],
    counter: u64,
    block: [u8; 32],
    offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedError {
    ZeroVersion,
    EmptySetSeed,
    EmptyCardTypeSeed,
}

impl HashStream {
    pub fn new(version: u32, set_seed: &[u8], card_type_seed: &[u8]) -> Result<Self, SeedError> {
        if version == 0 {
            return Err(SeedError::ZeroVersion);
        }
        if set_seed.is_empty() {
            return Err(SeedError::EmptySetSeed);
        }
        if card_type_seed.is_empty() {
            return Err(SeedError::EmptyCardTypeSeed);
        }

        let mut input =
            Vec::with_capacity(GENERATOR_DOMAIN.len() + set_seed.len() + card_type_seed.len() + 12);
        input.extend_from_slice(GENERATOR_DOMAIN);
        input.extend_from_slice(&version.to_be_bytes());
        input.extend_from_slice(
            &u32::try_from(set_seed.len())
                .expect("seed length exceeds u32")
                .to_be_bytes(),
        );
        input.extend_from_slice(set_seed);
        input.extend_from_slice(
            &u32::try_from(card_type_seed.len())
                .expect("seed length exceeds u32")
                .to_be_bytes(),
        );
        input.extend_from_slice(card_type_seed);

        Ok(Self {
            key: Sha256::digest(&input),
            counter: 0,
            block: [0; 32],
            offset: 32,
        })
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.fill(&mut bytes);
        u32::from_be_bytes(bytes)
    }

    /// Returns an unbiased value in `0..bound` using rejection sampling.
    pub fn next_bounded(&mut self, bound: u32) -> Option<u32> {
        if bound == 0 {
            return None;
        }
        let limit = (u64::from(u32::MAX) + 1) / u64::from(bound) * u64::from(bound);
        loop {
            let value = self.next_u32();
            if u64::from(value) < limit {
                return Some(value % bound);
            }
        }
    }

    fn fill(&mut self, output: &mut [u8]) {
        for byte in output {
            if self.offset == self.block.len() {
                let mut input = Vec::with_capacity(STREAM_DOMAIN.len() + 40);
                input.extend_from_slice(STREAM_DOMAIN);
                input.extend_from_slice(&self.key);
                input.extend_from_slice(&self.counter.to_be_bytes());
                self.block = Sha256::digest(&input);
                self.counter = self.counter.checked_add(1).expect("hash stream exhausted");
                self.offset = 0;
            }
            *byte = self.block[self.offset];
            self.offset += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_inputs_produce_same_stream() {
        let mut left = HashStream::new(1, &[0; 32], &[0x42]).unwrap();
        let mut right = HashStream::new(1, &[0; 32], &[0x42]).unwrap();
        for _ in 0..100 {
            assert_eq!(left.next_u32(), right.next_u32());
        }
    }

    #[test]
    fn stream_has_a_fixed_vector() {
        let mut stream = HashStream::new(1, &[0; 32], &[0]).unwrap();
        assert_eq!(stream.next_u32(), 0x24c9_1d27);
        assert_eq!(stream.next_u32(), 0xf782_1b1b);
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        assert_eq!(
            HashStream::new(0, &[1], &[1]).unwrap_err(),
            SeedError::ZeroVersion
        );
        assert_eq!(
            HashStream::new(1, &[], &[1]).unwrap_err(),
            SeedError::EmptySetSeed
        );
        assert_eq!(
            HashStream::new(1, &[1], &[]).unwrap_err(),
            SeedError::EmptyCardTypeSeed
        );
        assert_eq!(
            HashStream::new(1, &[1], &[1]).unwrap().next_bounded(0),
            None
        );
    }
}
