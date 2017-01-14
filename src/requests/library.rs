use super::*;

create_request! {
    LibrarySongsGet,
    "alibaba.xiami.api.library.songs.get",
    optional {
        page: isize,
        limit: isize,
        type_value: bool
    }
}

create_request! {
    LibraryCollectsGet,
    "alibaba.xiami.api.library.collects.get",
    required {
        type_value: isize
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    LibraryCollectAdd,
    "alibaba.xiami.api.library.collect.add",
    required {
        id: isize
    }
}

create_request! {
    LibraryCollectRemove,
    "alibaba.xiami.api.library.collect.remove",
    required {
        id: isize
    }
}

create_request! {
    LibrarySongAdd,
    "alibaba.xiami.api.library.song.add",
    required {
        id: isize
    }
}

create_request! {
    LibrarySongRemove,
    "alibaba.xiami.api.library.song.remove",
    required {
        id: isize
    }
}

create_request! {
    LibraryArtistAdd,
    "alibaba.xiami.api.library.artist.add",
    required {
        id: isize
    }
}

create_request! {
    LibraryArtistRemove,
    "alibaba.xiami.api.library.artist.remove",
    required {
        id: isize
    }
}

create_request! {
    LibraryAlbumAdd,
    "alibaba.xiami.api.library.album.add",
    required {
        id: isize
    }
}

create_request! {
    LibraryAlbumRemove,
    "alibaba.xiami.api.library.album.remove",
    required {
        id: isize
    }
}

create_request! {
    LibraryAlbumsGet,
    "alibaba.xiami.api.library.albums.get",
    required {
        limit: isize,
        page: isize
    }
}

create_request! {
    LibraryArtistsGet,
    "alibaba.xiami.api.library.artists.get",
    required {
        limit: isize,
        page: isize
    }
}
