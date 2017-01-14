use super::*;

create_request! {
    RankNewAlbumGet,
    "alibaba.xiami.api.rank.newAlbum.get",
    required {
        type_value: String
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    RankSongsGet,
    "alibaba.xiami.api.rank.songs.get",
    required {
        type_value: String
    }
}

create_request! {
    RankIndexGet,
    "alibaba.xiami.api.rank.index.get"
}

create_request! {
    RankNewAlbumsGet,
    "alibaba.xiami.api.rank.newAlbums.get",
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    RankWeekHotAlbumsGet,
    "alibaba.xiami.api.rank.weekhotalbums.get",
    required {
        type_value: String
    }
    optional {
        page: isize,
        limit: isize
    }
}

create_request! {
    RankPromotionAlbumsGet,
    "alibaba.xiami.api.rank.promotion.albums.get",
    required {
        type_value: String,
        limit: isize,
        page: isize
    }
}


