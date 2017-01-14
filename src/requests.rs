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

create_request! {
    SongDetailGet,
    "alibaba.xiami.api.song.detail.get",
    required {
        id: isize
    }
}

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
    RankIndexGet,
    "alibaba.xiami.api.rank.index.get"
}

create_request! {
    RankNewAlbumsGet,
    "alibaba.xiami.api.rank.newAlbums.get",
    optional {
        limit: isize,
        page: isize
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
    RankWeekHotAlbumsGet,
    "alibaba.xiami.api.rank.weekhotalbums.get",
    required {
        type_value: String
    }
    optional {
        page: isize,
        limit: isize
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

create_request! {
    RankPromotionAlbumsGet,
    "alibaba.xiami.api.rank.promotion.albums.get",
    required {
        type_value: String,
        limit: isize,
        page: isize
    }
}

create_request! {
    TagGenrelistGet,
    "alibaba.xiami.api.tag.genrelist.get"
}

create_request! {
    SearchHotwordsGet,
    "alibaba.xiami.api.search.hotwords.get"
}

create_request! {
    BannerGet,
    "alibaba.xiami.api.banner.get"
}

create_request! {
    MusicSceneGet,
    "alibaba.xiami.api.music.scene.get"
}

create_request! {
    RadioMusicdetailGet,
    "alibaba.xiami.api.radio.musicdetail.get",
    required {
        id: isize
    }
    optional {
        type_value: isize,
        limit: isize,
        context: String
    }
}

create_request! {
    MobileConfigureGet,
    "alibaba.xiami.api.mobile.configure.get"
}

create_request! {
    MvDetailGet,
    "alibaba.xiami.api.mv.detail.get",
    optional {
        mv_id: String
    }
}

create_request! {
    MvMusiclistGet,
    "alibaba.xiami.api.mv.musiclist.get",
    optional {
        type_value: String,
        order: String,
        limit: isize,
        page: isize
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

create_request! {
    SearchLetterGet,
    "alibaba.xiami.api.search.letter.get",
    optional {
        key: String
    }
}

create_request! {
    MobileSplashImageGet,
    "alibaba.xiami.api.mobile.splashimage.get",
    optional {
        type_value: isize,
        isnew: String,
        av: String
    }
}

create_request! {
    MobileFigureImageGet,
    "alibaba.xiami.api.mobile.figureimage.get",
    optional {
        limit: isize,
        type_value: String,
        av: String,
        device_type: String,
        device_id: String
    }
}

create_request! {
    PlaylogAdd,
    "alibaba.xiami.api.playlog.add",
    optional {
        id: isize,
        type_value: isize,
        time: isize,
        vip_role: isize,
        device_id: String
    }
}

create_request! {
    OperatorUniconGettelandStatus,
    "alibaba.alimusic.operator.unicom.gettelandstatus",
    optional {
        request_str: String,
        unikey: String
    }
}
