use crate::rpc::RPCError;
use hyper::header::HeaderValue;
use once_cell::sync::OnceCell;

static HEADER_FACTORY: OnceCell<HeaderFactory> = OnceCell::new();

pub(in crate::rpc) struct HeaderFactory {
    pub application_json: HeaderValue,
}

impl HeaderFactory {
    pub fn get_instance() -> Result<&'static HeaderFactory, RPCError> {
        let header_factory =
            HEADER_FACTORY.get_or_try_init(|| -> Result<_, RPCError> {
                let f = HeaderFactory {
                    application_json: HeaderValue::from_str(
                        "application/json",
                    )?,
                };

                Ok(f)
            })?;

        Ok(header_factory)
    }
}
