use super::*;

create_request! {
    CollectRecommendGet,
    "alibaba.xiami.api.collect.recommend.get",
    optional {
        page: isize,
        limit: isize,
        type_value: String,
        description: String
    }
}

create_request! {
    CollectDetailGet,
    "alibaba.xiami.api.collect.detail.get",
    required {
        id: isize
    }
    optional {
        full_des: bool,
        tag: String
    }
}
