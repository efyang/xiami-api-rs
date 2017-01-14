use super::*;

create_request! {
    AlbumDetailGet,
    "alibaba.xiami.api.album.detail.get",
    required {
        id: isize
    }
    optional {
        full_des: bool
    }
}
