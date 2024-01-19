use std::borrow::Cow;

pub trait LocalizedEnum {
    fn localize_enum() -> Cow<'static, str>;
    fn localize_variant(&self) -> Cow<'_, str>;
}

#[macro_export]
macro_rules! localized_enum {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident = $enum_tkey:literal {
            $($(#[$attr:meta])* $variant:ident = $tkey:literal,)*
        }
    ) => {
        $(#[$enum_attr])*
        $vis enum $name {
            $($(#[$attr])* $variant,)*
        }

        impl $crate::i18n::LocalizedEnum for $name {
            #[allow(unused)]
            fn localize_enum() -> std::borrow::Cow<'static, str> {
                t!($enum_tkey)
            }

            fn localize_variant(&self) -> std::borrow::Cow<'_, str> {
                t!(match self {
                    $($name::$variant => concat!($enum_tkey, ".", $tkey),)*
                })
            }
        }
    }
}
