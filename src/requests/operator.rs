use super::*;

create_request! {
    OperatorUniconGettelandStatus,
    "alibaba.alimusic.operator.unicom.gettelandstatus",
    optional {
        request_str: String,
        unikey: String
    }
}
