#![cfg(test)]
// miscellaneous tests that don't fit in other modules

#[test]
fn test_localization_fallback() {
    assert_eq!("Favorite", t!("sound.button.favorite.add"));
    assert_eq!("Favourite", t!("sound.button.favorite.add", locale = "en_GB"));
}

#[test]
fn test_localization_placeholders() {
    assert_eq!("Total files: 123", t!("stats.library.files", files = 123));
    assert_eq!("Dateianzahl: 123", t!("stats.library.files", locale = "de_AT", files = 123));
}

#[test]
fn test_nonexistent_translation_keys() {
    rust_i18n::set_locale("en_US"); // default is "en"
    assert_eq!("en_US.nonexistent.key", t!("nonexistent.key"));
    assert_eq!("nl_NL.nonexistent.key", t!("nonexistent.key", locale = "nl_NL"));
}

#[test]
fn test_missing_placeholders() {
    assert_eq!("ID: %{id}", t!("sound.info.id"));
    assert_eq!("ID: %{id}", t!("sound.info.id", locale = "en_GB"));
}
