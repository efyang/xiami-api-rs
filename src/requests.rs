pub trait XiamiRequest {
    fn get_url(&self) -> &str;
    fn get_params(&self) -> Vec<(&'static str, Parameter)>;
}

#[derive(Clone)]
pub enum Parameter {
    String(String),
    Number(isize),
}

impl Into<Parameter> for isize {
    fn into(self) -> Parameter { Parameter::Number(self) }
}

impl Into<Parameter> for String {
    fn into(self) -> Parameter { Parameter::String(self) }
}

impl Into<String> for Parameter {
    fn into(self) -> String {
        match self {
            Parameter::String(s) => s,
            Parameter::Number(i) => i.to_string(),
        }
    }
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! type_to_param_type {
    ($name:ident, isize) => {
        (stringify!($name), Some(Parameter::Number($name)))
    };
    ($name:ident, String) => {
        (stringify!($name), Some(Parameter::String($name)))
    };
}

macro_rules! generate_request_new {
    ($name:ident, $($required:ident: $requiredtype:ident),*) => {
        fn new($($required : $requiredtype),*) -> $name<'a> {
            $name {
                params: {
                    let mut tmp = [$(type_to_param_type!($required, $requiredtype)),*];
                    tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                    tmp
                },
            }
        }
    }
}

macro_rules! request_setter_body {
    ($this:expr, $optional:ident) => {
        {
            let index = $this.params.binary_search_by_key(&stringify!($optional), |&(name, _)| name).unwrap();
            $this.params[index].1 = Some($optional.into());
            $this
        }
    }
}

macro_rules! generate_request_setters {
    ($name:ident, $($optional:ident: $optionaltype:ident),*) => {
        $(
            interpolate_idents! {
                fn [with_ $optional](&mut self, $optional: $optionaltype) -> &mut $name {
                    request_setter_body!(self, $optional)
                }
            }
         )*
    }
}

macro_rules! parse_mappings {
    ($key:ident : $ty:ident) => {
        $key:$ty => stringify!($key)
    };
    ($key:ident : $ty:ident => $key_alt:expr) => {
        $key:$ty => $key_alt
    };
}

macro_rules! extract {
    (key $key:ident:$ty:ident => $key_alt:expr) => ($key);
    (ty $key:ident:$ty:ident => $key_alt:expr) => ($ty);
    (key_alt $key:ident:$ty:ident => $key_alt:expr) => ($key_alt);
}

macro_rules! create_request {
    {
        $name:ident,
        $url:expr,
        optional { $($optional:ident : $optionaltype:ident),* }
        required { $($required:ident : $requiredtype:ident),* }
    } => {
        pub struct $name {
            params: [(&'static str, Option<Parameter>); count!($($optional)*) + count!($($required)*)]
        }

        impl $name {
            pub fn new($($required : $requiredtype),*) -> $name {
                $name {
                    params: {
                        let mut tmp = [$(type_to_param_type!($required, $requiredtype)),*, $((stringify!($optional), None))*];
                        tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                        tmp
                    },
                }
            }
            generate_request_setters!($name, $($optional:$optionaltype)*);
        }

        impl XiamiRequest for $name {
            fn get_url(&self) -> &'static str {
                $url
            }

            fn get_params(&self) -> Vec<(&'static str, Parameter)> {
                self.params
                    .iter()
                    .filter(|&&(_, ref p)| p.is_some())
                    .map(|&(name, ref p)| (name, p.clone().unwrap()))
                    .collect::<Vec<_>>()
            }
        }
    }
}

create_request! {
    RadioSongsGet,
    "alibaba.xiami.api.radio.songs.get",
    optional {
        limit: isize
    }
    required {
        type_value: isize,
        oid: isize
    }
}


