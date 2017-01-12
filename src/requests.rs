pub trait XiamiRequest {
    fn get_url(&self) -> &str;
    fn get_fields(&self) -> &[Parameter];
}

pub enum Parameter {
    String(String),
    Number(isize),
}

// how to macro this???
pub struct RadioInfoRequest {}

impl XiamiRequest for RadioInfoRequest {
    fn get_url(&self) -> &str {
        "alibaba.xiami.api.radio.info"
    }

    fn get_fields(&self) -> &[Parameter] {
        unimplemented!()
    }
}
