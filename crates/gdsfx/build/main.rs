mod i18n;
mod credits;
mod icon;
mod libs;

fn main() {
    i18n::build();
    credits::build();
    icon::build();
    libs::build();
}
