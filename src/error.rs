use top_sdk::{TopError};

quick_error! {
    #[derive(Debug)]
    pub enum XiamiError {
        /// error in underlying TopClient
        TopError(err: TopError) {
            from(err: TopError) -> (err)
        }
    }
}

pub type Result<R> = ::std::result::Result<R, XiamiError>;
