macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! convert_type_name {
    (type_value) => ("type");
    ($name:ident) => (stringify!($name));
}

macro_rules! type_to_param_type {
    ($name:ident, isize) => {
        (convert_type_name!($name), Some(Parameter::Number($name)))
    };
    ($name:ident, String) => {
        (convert_type_name!($name), Some(Parameter::String($name)))
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
    ($this:expr, type_value) => {
        {
            let index = $this.params.binary_search_by_key(&"type", |&(name, _)| name).unwrap();
            $this.params[index].1 = Some(type_value.into());
            $this
        }
    };
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
                pub fn [with_ $optional](&mut self, $optional: $optionaltype) -> &mut $name {
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

#[macro_export]
macro_rules! create_request {
    {
        $name:ident,
        $url:expr,
        required { $($required:ident : $requiredtype:ident),* }
        optional { $($optional:ident : $optionaltype:ident),* }
    } => {
        pub struct $name {
            params: [(&'static str, Option<Parameter>); count!($($optional)*) + count!($($required)*)]
        }

        impl $name {
            pub fn new($($required : $requiredtype),*) -> $name {
                $name {
                    params: {
                        let mut tmp = [$(type_to_param_type!($required, $requiredtype)),*, $((convert_type_name!($optional), None)),*];
                        tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                        tmp
                    },
                }
            }
            generate_request_setters!($name, $($optional:$optionaltype),*);
        }

        impl XiamiRequest for $name {
            fn url(&self) -> &'static str {
                $url
            }

            fn params(&self) -> Vec<(&'static str, Parameter)> {
                self.params
                    .iter()
                    .filter(|&&(_, ref p)| p.is_some())
                    .map(|&(name, ref p)| (name, p.clone().unwrap()))
                    .collect::<Vec<_>>()
            }
        }
    };
    {
        $name:ident,
        $url:expr
    } => {
        pub struct $name {}

        impl $name {
            pub fn new() -> $name {
                $name {}
            }
        }

        impl XiamiRequest for $name {
            fn url(&self) -> &'static str {
                $url
            }

            fn params(&self) -> Vec<(&'static str, Parameter)> {
                Vec::new()
            }
        }
    }
}
