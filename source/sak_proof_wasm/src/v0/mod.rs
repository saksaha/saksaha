// #[cfg(test)]
// mod g16;
mod proof;

pub type ProofWasmError = Box<dyn std::error::Error + Send + Sync>;
