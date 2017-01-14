use super::*;

create_request! {
    SearchSongsGet,
    "alibaba.xiami.api.search.songs.get",
    required {
        key: String
    }
    optional {
        page: isize,
        limit: isize,
        is_pub: String, // this should be a more specific variant
        category: String
    }
}

create_request! {
    SearchSummaryGet,
    "alibaba.xiami.api.search.summary.get",
    required {
        key: String
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    SearchAlbumsGet,
    "alibaba.xiami.api.search.albums.get",
    required {
        key: String
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    SearchArtistsGet,
    "alibaba.xiami.api.search.artists.get",
    required {
        key: String
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    SearchCollectsGet,
    "alibaba.xiami.api.search.collects.get",
    optional {
        limit: isize,
        page: isize,
        key: String,
        tag: String,
        order: String
    }
}

create_request! {
    SearchHotwordsGet,
    "alibaba.xiami.api.search.hotwords.get"
}

create_request! {
    SearchLetterGet,
    "alibaba.xiami.api.search.letter.get",
    optional {
        key: String
    }
}
