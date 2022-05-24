use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Status {
    code: u16,
    message: String,
    data: Option<String>,
}

impl Status {
    // pub const SUCCESS: Status = Status {
    //     code: 2000,
    //     message: String::from("SUCCESS"),
    // };

    // pub fn success(data: Option<String>) {
    //     let data = match data {
    //         Ok(data) => {
    //             Status {
    //                 code: 2000,
    //                 message: String::from("Success"),
    //                 data
    //             }
    //         }
    //     }
    // }

    pub fn success_response() -> Status {
        Status {
            code: 200,
            message: String::from("Success"),
            data: None,
        }
    }
}
