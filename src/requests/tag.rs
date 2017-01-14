use super::*;

create_request! {
    TagTags,
    "alibaba.xiami.api.tag.tags",
    required {
        object_id: isize,
        object_type: String
    }
}

create_request! {
    TagGenreAlbumGet,
    "alibaba.xiami.api.tag.genre.album.get",
    required {
        type_value: isize,
        id: isize
    }
    optional {
        page: isize,
        limit: isize
    }
}

create_request! {
    TagGenreArtistGet,
    "alibaba.xiami.api.tag.genre.artist.get",
    required {
        id: isize,
        type_value: isize
    }
    optional {
        page: isize,
        limit: isize
    }
}

create_request! {
    TagGenreSongGet,
    "alibaba.xiami.api.tag.genre.song.get",
    required {
        type_value: String,
        id: isize
    }
    optional {
        page: isize,
        limit: isize
    }
}

create_request! {
    TagGenrelistGet,
    "alibaba.xiami.api.tag.genrelist.get"
}
