pub(crate) const VALIDATOR: &[u8] = include_bytes!("./sak_ctrt_validator.wasm");

pub(crate) const ALLOC_FN: &str = "alloc";

pub(crate) const MEMORY: &str = "memory";

pub(crate) const DEFAULT_VALIDATOR_HASHMAP_CAPACITY: usize = 10;
