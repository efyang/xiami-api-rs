use super::*;

create_request! {
    ArtistDetailGet,
    "alibaba.xiami.api.artist.detail.get",
    required {
        id: isize
    }
    optional {
        description: String
    }
}

create_request! {
    ArtistHotSongsGet,
    "alibaba.xiami.api.artist.hotSongs.get",
    required {
        id: isize
    }
}

create_request! {
    ArtistAlbumsGet,
    "alibaba.xiami.api.artist.albums.get",
    required {
        id: isize
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    ArtistPromotionsGet,
    "alibaba.xiami.api.artist.promotions.get",
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    ArtistWordbookGet,
    "alibaba.xiami.api.artist.wordbook.get",
    required {
        type_value: String
    }
    optional {
        page: isize,
        limit: isize
    }
}

create_request! {
    ArtistSimilarGet,
    "alibaba.xiami.api.artist.similar.get",
    required {
        id: isize,
        limit: isize
    }
}

create_request! {
    ArtistMusiclistGet,
    "alibaba.xiami.api.artist.musiclist.get",
    optional {
        type_value: String,
        order: String
    }
}

