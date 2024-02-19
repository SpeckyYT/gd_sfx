const THEMES_FOLDER: &str = "../themes/doki-master-theme/definitions";

const DOKI_OUTPUT_FILE: &str = "doki_themes.rs";

use walkdir::WalkDir;
use gdsfx_build::{write_output_rust, TokenStream};
use gdsfx_files::read_json_file;
use quote::format_ident;
use serde_json::{Map, Value};

pub fn build() {
    let themes = WalkDir::new(THEMES_FOLDER)
        .into_iter()
        .filter_map(|path| path.ok().and_then(|p| read_json_file::<Map<String, Value>>(p.path()).ok()))
        .map(|content| {
            let name = content["name"].as_str().unwrap();
            let group = content["group"].as_str().unwrap();
            let author = content["author"].as_str().unwrap();
            let colors = content["colors"].as_object().unwrap();

            let const_ident_name = format_ident!("{}", format!("{group}_{name}").replace(|c: char| !c.is_ascii_alphabetic(), "_").replace("__", "_").replace("__", "_").to_uppercase());
            let variant_name = format_ident!("{}", format!("{}{}{name}", group.chars().nth(0).unwrap().to_uppercase(), &group[1..]).replace(|c: char| !c.is_ascii_alphabetic(), ""));

            macro_rules! get_colors {
                ($($input:ident => $output:ident,)*) => { {
                    let keys = [
                        $(format_ident!("{}", stringify!($output))),*
                    ];
                    let [r,g,b] = [
                        $({
                            let string = colors.get(stringify!($input))
                                .and_then(|output| output.as_str())
                                .unwrap_or("#abcdef");
                            let colors = [
                                u8::from_str_radix(&string[1..3], 16),
                                u8::from_str_radix(&string[3..5], 16),
                                u8::from_str_radix(&string[5..7], 16),
                            ];
                            colors.map(|color| color.unwrap())
                        }),*
                    ]
                    .into_iter()
                    .fold([vec![],vec![],vec![]], |[mut r, mut g, mut b], [nr,ng,nb]| {
                        r.push(nr);
                        g.push(ng);
                        b.push(nb);
                        [r,g,b]
                    });
                    quote::quote! {
                        pub const #const_ident_name: Theme = Theme {
                            author: #author,
                            #(
                                #keys: Color32::from_rgb(#r, #g, #b),
                            )*
                        };
                    }
                } };
            }

            let tokens = get_colors!{
                caretRow => rosewater,
                accentColor => flamingo,
                startColor => pink,
                searchForeground => mauve,
                fileRed => red,
                accentContrastColor => maroon,
                fileOrange => peach,
                fileYellow => yellow,
                keyColor => green,
                keywordColor => teal,
                iconAccent => sky,
                iconAccentCompliment => sapphire,
                iconBaseBlend => blue,
                buttonFont => lavender,
                searchBackground => text,
                selectionForeground => subtext1,
                selectionBackground => subtext0,
                identifierHighlight => overlay2,
                selectionInactive => overlay1,
                borderColor => overlay0,
                buttonColor => surface2,
                baseBackground => surface1,
                headerColor => surface0,
                secondaryBackground => base,
                inactiveBackground => mantle,
                inactiveBackgroundDarker => crust,
            };

            (const_ident_name, variant_name, tokens)
        })
        .collect::<Vec<_>>();

    let definition_tokens = themes.iter()
        .fold(TokenStream::new(), |mut tokens, (_, _, new_tokens)| {
            tokens.extend(new_tokens.clone());
            tokens
        });

    let const_names = themes.iter().map(|a| a.0.clone()).collect::<Vec<_>>();
    let variant_names = themes.iter().map(|a| a.1.clone()).collect::<Vec<_>>();
    
    let const_macro = quote::quote! {
        macro_rules! const_macro {
            () => {
                #(#const_names,)*
            }
            pub(crate) use theme;
        }
    };
    let variants_macro = quote::quote! {
        macro_rules! variants_macro {
            () => {
                #(#variant_names,)*
            }
            pub(crate) use theme;
        }
    };

    let mut tokens = definition_tokens;

    tokens.extend(const_macro);
    tokens.extend(variants_macro);

    write_output_rust(DOKI_OUTPUT_FILE, tokens);
}
