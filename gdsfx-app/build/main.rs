mod i18n;
mod credits;
mod icon;

fn main() {
    i18n::build();
    credits::build();
    icon::build();
}
