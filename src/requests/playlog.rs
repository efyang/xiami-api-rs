use super::*;

create_request! {
    PlaylogAdd,
    "alibaba.xiami.api.playlog.add",
    optional {
        id: isize,
        type_value: isize,
        time: isize,
        vip_role: isize,
        device_id: String
    }
}
