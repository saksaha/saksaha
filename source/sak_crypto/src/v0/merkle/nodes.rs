use crate::CryptoError;
use crate::Scalar;

pub struct MerkleNodes {
    height: u32,
}

impl MerkleNodes {
    pub fn new(height: u32) -> MerkleNodes {
        MerkleNodes { height }
    }

    pub fn hydrate(&self, nodes: &[(&str, Scalar)]) -> Result<(), CryptoError> {
        if nodes.len() != self.height as usize {
            return Err(format!(
                "Merkle nodes of length ({}) should be \
                provided, len: {}",
                self.height,
                nodes.len(),
            )
            .into());
        }

        Ok(())
    }
}
