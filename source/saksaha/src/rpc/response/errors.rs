use serde::ser::{Serialize as SerSerialize, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub(crate) struct HandleError2<'a> {
    pub code: &'a str,
    pub desc: &'a str,
    // pub msg: &'a String,
}

#[derive(Debug)]
pub(crate) enum HandleError {
    ResHeaderParseFail,
    ResSerializeFail,
    SendTxFail { msg: String },
    ReqParamInvalid { msg: String },
    ParseReqFail { msg: String },
}

// fn get_err_meta(err: &HandleError) -> (&'static str, &'static str, String) {
//     match err {
//         HandleError::ResHeaderParseFail => {
//             ("000001", "Response header parse fail", String::default())
//         }
//         HandleError::ResSerializeFail => {
//             ("000002", "Response serialization fail", String::default())
//         }
//         HandleError::SendTxFail { msg } => ("000003", "Cannot send tx", msg),
//         HandleError::ReqParamInvalid { msg } => {
//             ("000004", "Request parameter invalid", msg)
//         }
//         HandleError::ParseReqFail { msg } => {
//             ("000005", "Failed to parse request in bytes", msg)
//         }
//     }
// }

// impl Serialize for HandleError {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let (code, desc, msg) = get_err_meta(&self);

//         let mut state = serializer.serialize_struct("Error", 2)?;
//         state.serialize_field("code", code)?;
//         state.serialize_field("desc", desc)?;
//         state.serialize_field("msg", msg)?;
//         state.end()
//     }
// }
