pub type InvokeResult = Vec<u8>;

pub type ContractError = Box<dyn std::error::Error + Send + Sync>;

pub const ERROR_PLACEHOLDER: [u8; 6] = [1, 2, 3, 4, 5, 6];

pub fn make_error_vec(err: ContractError) -> Vec<u8> {
    let v = err.to_string().as_bytes().to_vec();

    [ERROR_PLACEHOLDER.to_vec(), v].concat()
}
