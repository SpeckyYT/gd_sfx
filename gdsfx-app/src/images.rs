use crate::egui::{include_image, ImageSource};

macro_rules! images {
    { $($name:ident: $path:literal)* } => {
        $(
            pub const $name: ImageSource<'_> = include_image!(
                gdsfx_files::workspace_path!(concat!("assets/", $path))
            );
        )*
    }
}

images! {
    FAVORITE_STAR: "twemoji-white-medium-star.png"
    DOWNLOAD: "svg/download-solid.svg"
    TRASH: "svg/trash-can-regular.svg"
    PLAY: "svg/play-solid.svg"
    STOP: "svg/stop-solid.svg"
    STAR_SOLID: "svg/star-solid.svg"
    STAR_REGULAR: "svg/star-regular.svg"
    MAGNIFYING_GLASS: "svg/magnifying-glass-solid.svg"
    TOOLS: "svg/screwdriver-wrench-solid.svg"
    CHART: "svg/chart-simple-solid.svg"
    GEAR: "svg/gear-solid.svg"
    PEOPLE_GROUP: "svg/people-group-solid.svg"
    RIGHT_TO_BRACKET: "svg/right-to-bracket-solid.svg"
}
