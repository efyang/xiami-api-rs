use super::*;

create_request! {
    RecommendDailySongsGet,
    "alibaba.xiami.api.recommend.dailySongs.get",
    optional {
        limit: isize,
        ids: String
    }
}
