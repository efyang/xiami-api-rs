use top_sdk::*;
use std::path::Path;
use error::*;
use requests::XiamiRequest;

pub struct XiamiClient {
    top_client: TaobaoClient,
}

impl XiamiClient {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(url: S, appkey: S, secret: S, capath: P) -> Result<XiamiClient> {
        set_capath(capath)?;
        Ok(XiamiClient {
            top_client: {
                TaobaoClient::new(url, appkey, secret)?
            }
        })
    }

    pub fn execute_request<R: XiamiRequest>(request: R) -> Result<()> {
        let mut req = TopRequest::new();
        req.set_api_name(request.api())?;
        for (key, value) in request.params().into_iter().map(|(k, v)| (k, v.into_string())) {
            req.add_param(key, &value)?;
        }
        Ok(())
    }
}
