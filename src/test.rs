use crate::util;

#[test]
fn stringify_duration() {
    assert_eq!("0.00",  util::stringify_duration(0));
    assert_eq!("0.12",  util::stringify_duration(12));
    assert_eq!("3.45",  util::stringify_duration(345));
    assert_eq!("67.89", util::stringify_duration(6789));
    
    assert_eq!("0.01",  util::stringify_duration(1));
    assert_eq!("0.10",  util::stringify_duration(10));
    assert_eq!("1.00",  util::stringify_duration(100));
}

#[test]
fn localization_fallback() {
    assert_eq!("Favorite", t!("sound.button.favorite.add"));
    assert_eq!("Favourite", t!("sound.button.favorite.add", locale = "en_GB"));
}

#[test]
fn localization_placeholders() {
    assert_eq!("Total files: 123", t!("stats.library.files", files = 123));
    assert_eq!("Total files: 123", t!("stats.library.files", locale = "en_GB", files = 123));
}

#[test]
fn nonexistent_translation_keys() {
    rust_i18n::set_locale("en_US"); // default is "en"
    assert_eq!("en_US.nonexistent.key", t!("nonexistent.key"));
    assert_eq!("nl_NL.nonexistent.key", t!("nonexistent.key", locale = "nl_NL"));
}

#[test]
fn format_locale() {
    assert_eq!("English (United States)", util::format_locale("en_US"));
    assert_eq!("Deutsch (Österreich)", util::format_locale("de_AT"));
}
