use top_sdk::*;
use std::path::Path;
use error::*;
use requests::XiamiRequest;

pub struct XiamiClient {
    session: Option<String>,
    top_client: TaobaoClient,
}

impl XiamiClient {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(url: S,
                                              appkey: S,
                                              secret: S,
                                              capath: P)
                                              -> Result<XiamiClient> {
        set_capath(capath)?;
        Ok(XiamiClient {
            session: None,
            top_client: {
                TaobaoClient::new(url, appkey, secret)?
            },
        })
    }

    pub fn execute_request<R: XiamiRequest>(&mut self, request: R) -> Result<()> {
        let mut req = TopRequest::new();
        req.set_api_name(request.api())?;
        for (key, value) in request.params().into_iter().map(|(k, v)| (k, v.into_string())) {
            req.add_param(key, &value)?;
        }
        let (new_session, raw_responses) = self.top_client.execute(&mut req, self.session.clone())?;
        self.session = new_session;
        for raw_response in raw_responses {
            let (key, value) = raw_response?;
            println!("{} : {}", key, value);
        }
        Ok(())
    }
}
