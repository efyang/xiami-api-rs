use top_sdk::TopRequest;

pub trait XiamiRequest {
    fn url(&self) -> &str;
    fn params(&self) -> Vec<(&'static str, Parameter)>;
}

#[derive(Clone)]
pub enum Parameter {
    String(String),
    Number(isize),
    Boolean(bool),
}

macro_rules! auto_impl_param {
    ($ty:ident, $variant:path) => {
        impl Into<Parameter> for $ty {
            fn into(self) -> Parameter {
                $variant(self)
            }
        }
    }
}

auto_impl_param!(String, Parameter::String);
auto_impl_param!(isize, Parameter::Number);
auto_impl_param!(bool, Parameter::Boolean);

impl Into<String> for Parameter {
    fn into(self) -> String {
        match self {
            Parameter::String(s) => s,
            Parameter::Number(i) => i.to_string(),
            Parameter::Boolean(b) => b.to_string(),
        }
    }
}

create_request! {
    RadioInfo,
    "alibaba.xiami.api.radio.info"
}

create_request! {
    RadioSongsGet,
    "alibaba.xiami.api.radio.songs.get",
    required {
        type_value: isize,
        oid: isize
    }
    optional {
        limit: isize
    }
}

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
    RadioMyselfGet,
    "alibaba.xiami.api.radio.myself.get",
    optional {
        limit: isize
    }
}

create_request! {
    RadioGuessGet,
    "alibaba.xiami.api.radio.guess.get",
    optional {
        limit: isize
    }
}

create_request! {
    RadioPromotionsGet,
    "alibaba.xiami.api.radio.promotions.get",
    optional {
        limit: isize,
        page: isize
    }
}

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
    RecommendDailySongsGet,
    "alibaba.xiami.api.recommend.dailySongs.get",
    optional {
        limit: isize,
        ids: String
    }
}

create_request! {
    MemberAccountGet,
    "alibaba.xiami.api.member.account.get"
}

create_request! {
    LibrarySongsGet,
    "alibaba.xiami.api.library.songs.get",
    optional {
        page: isize,
        limit: isize,
        type_value: bool
    }
}
