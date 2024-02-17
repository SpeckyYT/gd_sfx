use std::fmt::Debug;

use eframe::egui;
use egui::Key;

type Keys = &'static [Key];
type Callback = &'static dyn Fn() -> ();

#[derive(Debug, Default)]
pub struct Konami {
    konami: Vec<KonamiString>,
}

impl Konami {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.konami.iter_mut()
            .for_each(|konami| konami.update(ctx));
    }
    pub fn push(&mut self, konami: KonamiString) {
        if !self.konami.contains(&konami) {
            self.konami.push(konami);
        }
    }
}

pub struct KonamiString {
    keys: Keys,
    index: (usize, bool),
    callback: Callback,
}

impl Debug for KonamiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KonamiString")
            .field("keys", &self.keys)
            .field("index", &self.index)
            .finish()
    }
}

impl PartialEq for KonamiString {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
}

impl KonamiString {
    pub const fn new(keys: Keys, callback: Callback) -> Self {
        if keys.is_empty() {
            panic!("Konami keys cannot be empty")
        }
        KonamiString {
            keys,
            index: (0, false),
            callback,
        }
    }
    pub fn update(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if self.index.1 == i.key_down(self.keys[self.index.0]) {
                if self.index.1 {
                    self.index.0 += 1;
                    if self.index.0 >= self.keys.len() {
                        self.index.0 = 0;
                        (&self.callback)();
                    }
                    self.index.1 = false;
                }
                self.index.1 = !self.index.1;
            }
        });
    }
}
