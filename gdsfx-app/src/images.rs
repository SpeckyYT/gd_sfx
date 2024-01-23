use crate::egui::include_image;
use crate::egui::ImageSource;

macro_rules! images {
    {$($name:ident: $path:literal)*} => {
        $(
            // I wanted to use a concat macro, but I wasn't able to implement it
            pub const $name: ImageSource<'static> = include_image!($path);
        )*
    }
}

images!{
    DOWNLOAD: "../../assets/svg/download-solid.svg"
    TRASH: "../../assets/svg/trash-can-regular.svg"
    PLAY: "../../assets/svg/play-solid.svg"
    STOP: "../../assets/svg/stop-solid.svg"
    STAR_SOLID: "../../assets/svg/star-solid.svg"
    STAR_REGULAR: "../../assets/svg/star-regular.svg"
}
