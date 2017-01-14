use super::*;

create_request! {
    MvDetailGet,
    "alibaba.xiami.api.mv.detail.get",
    optional {
        mv_id: String
    }
}

create_request! {
    MvMusiclistGet,
    "alibaba.xiami.api.mv.musiclist.get",
    optional {
        type_value: String,
        order: String,
        limit: isize,
        page: isize
    }
}
