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
    FAVORITE_STAR: "../../assets/twemoji-white-medium-star.png"
    DOWNLOAD: "../../assets/svg/download-solid.svg"
    TRASH: "../../assets/svg/trash-can-regular.svg"
    PLAY: "../../assets/svg/play-solid.svg"
    STOP: "../../assets/svg/stop-solid.svg"
    STAR_SOLID: "../../assets/svg/star-solid.svg"
    STAR_REGULAR: "../../assets/svg/star-regular.svg"
    MAGNIFYING_GLASS: "../../assets/svg/magnifying-glass-solid.svg"
    TOOLS: "../../assets/svg/screwdriver-wrench-solid.svg"
    CHART: "../../assets/svg/chart-simple-solid.svg"
    GEAR: "../../assets/svg/gear-solid.svg"
    PEOPLE_GROUP: "../../assets/svg/people-group-solid.svg"
}
