use crate::ContractError;

pub type InvokeResult = Vec<u8>;

pub const ERROR_PLACEHOLDER: [u8; 6] = [1, 2, 3, 4, 5, 6];

pub fn make_error_vec(err: ContractError, msg: &str) -> Vec<u8> {
    let err_str = err.to_string();
    let err_msg = format!("{} - {}", err_str, msg);
    let v = err_msg.as_bytes().to_vec();

    [ERROR_PLACEHOLDER.to_vec(), v].concat()
}
