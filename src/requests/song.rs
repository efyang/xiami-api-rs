use super::*;

create_request! {
    SongDetailGet,
    "alibaba.xiami.api.song.detail.get",
    required {
        id: isize
    }
}

create_request! {
    SongSongsByTagGet,
    "alibaba.xiami.api.song.songsByTag.get",
    required {
        tag: String
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    SongSimilarGet,
    "alibaba.xiami.api.song.similar.get",
    required {
        id: isize
    }
    optional {
        limit: isize
    }
}

