mod i18n;
mod credits;
mod icon;
mod themes;

fn main() {
    i18n::build();
    credits::build();
    icon::build();
    themes::build();
}
