use super::*;

create_request! {
    MobileConfigureGet,
    "alibaba.xiami.api.mobile.configure.get"
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
