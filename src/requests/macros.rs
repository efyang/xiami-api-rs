macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! convert_type_name {
    (type_value) => ("type");
    ($name:ident) => (stringify!($name));
}

macro_rules! type_to_param_type {
    ($name:ident) => {
        (convert_type_name!($name), Some($name.into()))
    };
}

macro_rules! generate_request_new {
    ($name:ident, $($required:ident: $requiredtype:ident),*) => {
        fn new($($required : $requiredtype),*) -> $name<'a> {
            $name {
                params: {
                    let mut tmp = [$(type_to_param_type!($required)),*];
                    tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                    tmp
                },
            }
        }
    }
}

macro_rules! check_type {
    ($optionala:ident, type_value) => ($optionala);
    ($optionala:ident, $optional:ident) => ($optionala);
}

macro_rules! request_setter_body {
    ($this:expr, $optional:ident) => {
        {
            let index = $this.params.binary_search_by_key(&stringify!($optional), |&(name, _)| name).unwrap();
            $this.params[index].1 = Some(check_type!($optional, $optional).into());
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

macro_rules! impl_req {
    ($name:ident, $api:expr) => {
        impl XiamiRequest for $name {
            fn api(&self) -> &'static str {
                $api
            }

            fn params(&self) -> Vec<(&'static str, Parameter)> {
                self.params
                    .iter()
                    .filter(|&&(_, ref p)| p.is_some())
                    .map(|&(name, ref p)| (name, p.clone().unwrap()))
                    .collect::<Vec<_>>()
            }
        }
    }
}

#[macro_export]
macro_rules! create_request {
    {
        $name:ident,
        $api:expr,
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
                        let mut tmp = [$(type_to_param_type!($required)),*, $((convert_type_name!($optional), None)),*];
                        tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                        tmp
                    },
                }
            }
            generate_request_setters!($name, $($optional:$optionaltype),*);
        }
        impl_req!($name, $api);
    };
    {
        $name:ident,
        $api:expr,
        optional { $($optional:ident : $optionaltype:ident),* }
    } => {
        pub struct $name {
            params: [(&'static str, Option<Parameter>); count!($($optional)*)]
        }

        impl $name {
            pub fn new() -> $name {
                $name {
                    params: {
                        let mut tmp = [$((convert_type_name!($optional), None)),*];
                        tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                        tmp
                    },
                }
            }
            generate_request_setters!($name, $($optional:$optionaltype),*);
        }
        impl_req!($name, $api);
    };
    {
        $name:ident,
        $api:expr,
        required { $($required:ident : $requiredtype:ident),* }
    } => {
        pub struct $name {
            params: [(&'static str, Option<Parameter>); count!($($required)*)]
        }

        impl $name {
            pub fn new($($required : $requiredtype),*) -> $name {
                $name {
                    params: {
                        let mut tmp = [$(type_to_param_type!($required)),*];
                        tmp.sort_by(|&(a, _), &(b, _)| a.cmp(b));
                        tmp
                    },
                }
            }
        }
        impl_req!($name, $api);
    };

    {
        $name:ident,
        $api:expr
    } => {
        pub struct $name {}

        impl $name {
            pub fn new() -> $name {
                $name {}
            }
        }

        impl XiamiRequest for $name {
            fn api(&self) -> &'static str {
                $api
            }

            fn params(&self) -> Vec<(&'static str, Parameter)> {
                Vec::new()
            }
        }
    }
}
