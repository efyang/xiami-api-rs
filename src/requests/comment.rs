use super::*;

create_request! {
    CommentCollectGet,
    "alibaba.xiami.api.comment.collect.get",
    required {
        id: isize
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    CommentAlbumGet,
    "alibaba.xiami.api.comment.album.get",
    required {
        id: isize
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    CommentSongGet,
    "alibaba.xiami.api.comment.song.get",
    required {
        id: isize
    }
    optional {
        limit: isize,
        page: isize
    }
}

create_request! {
    CommentArtistGet,
    "alibaba.xiami.api.comment.artist.get",
    required {
        id: isize
    }
    optional {
        page: isize,
        limit: isize
    }
}
