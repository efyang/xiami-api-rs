use top_sdk::TopRequest;

#[macro_use]
mod macros;
pub mod prelude;
pub mod rank;
pub mod artist;
pub mod album;
pub mod banner;
pub mod collect;
pub mod comment;
pub mod library;
pub mod member;
pub mod mobile;
pub mod music;
pub mod mv;
pub mod radio;
pub mod search;
pub mod operator;
pub mod song;
pub mod tag;
pub mod recommend;
pub mod playlog;

pub trait XiamiRequest {
    fn url(&self) -> &str;
    fn params(&self) -> Vec<(&'static str, Parameter)>;
}

#[derive(Clone)]
pub enum Parameter {
    String(String),
    Number(isize),
    Boolean(bool),
}

macro_rules! auto_impl_param {
    ($ty:ident, $variant:path) => {
        impl Into<Parameter> for $ty {
            fn into(self) -> Parameter {
                $variant(self)
            }
        }
    }
}

auto_impl_param!(String, Parameter::String);
auto_impl_param!(isize, Parameter::Number);
auto_impl_param!(bool, Parameter::Boolean);

impl Into<String> for Parameter {
    fn into(self) -> String {
        match self {
            Parameter::String(s) => s,
            Parameter::Number(i) => i.to_string(),
            Parameter::Boolean(b) => b.to_string(),
        }
    }
}
