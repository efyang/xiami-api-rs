use super::*;

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
    RadioMyselfGet,
    "alibaba.xiami.api.radio.myself.get",
    optional {
        limit: isize
    }
}

create_request! {
    RadioGuessGet,
    "alibaba.xiami.api.radio.guess.get",
    optional {
        limit: isize
    }
}

create_request! {
    RadioPromotionsGet,
    "alibaba.xiami.api.radio.promotions.get",
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    RadioMusicdetailGet,
    "alibaba.xiami.api.radio.musicdetail.get",
    required {
        id: isize
    }
    optional {
        type_value: isize,
        limit: isize,
        context: String
    }
}
