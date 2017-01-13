pub trait XiamiRequest {
    fn url(&self) -> &str;
    fn params(&self) -> Vec<(&'static str, Parameter)>;
}

#[derive(Clone)]
pub enum Parameter {
    String(String),
    Number(isize),
}

impl Into<Parameter> for isize {
    fn into(self) -> Parameter {
        Parameter::Number(self)
    }
}

impl Into<Parameter> for String {
    fn into(self) -> Parameter {
        Parameter::String(self)
    }
}

impl Into<String> for Parameter {
    fn into(self) -> String {
        match self {
            Parameter::String(s) => s,
            Parameter::Number(i) => i.to_string(),
        }
    }
}

create_request! {
    RadioInfo,
    "alibaba.xiami.api.radio.info"
}

create_request! {
    RadioSongsGet,
    "alibaba.xiami.api.radio.songs.get",
    required {
        type_value: isize,
        oid: isize
    }
    optional {
        limit: isize
    }
}

create_request! {
    SearchSongsGet,
    "alibaba.xiami.api.search.songs.get",
    required {
        key: String
    }
    optional {
        page: isize,
        limit: isize,
        is_pub: String, // this should be a more specific variant
        category: String
    }
 }
