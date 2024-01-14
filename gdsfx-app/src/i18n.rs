pub trait LocalizedEnum {
    fn localize_enum() -> String;
    fn localize_variant(&self) -> String;
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
            fn localize_variant(&self) -> String {
                t!(match self {
                    $($name::$variant => concat!($enum_tkey, ".", $tkey),)*
                })
            }

            #[allow(unused)]
            fn localize_enum() -> String {
                t!($enum_tkey)
            }
        }
    }
}
