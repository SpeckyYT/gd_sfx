use std::{fs, path::PathBuf, thread};

use eframe::epaint::ahash::{HashMap, HashMapExt};
use slab_tree::{TreeBuilder, NodeId, NodeRef};

use crate::{
    util::{*, encoding::full_decode, requests::{download_sfx, CDN_URL}},
    settings::{has_favourite, FAVOURITES_CHARACTER},
    stats::{add_file_to_stats, remove_file_from_stats}
};

pub mod gui;

#[derive(Debug, Clone)]
pub struct Library {
    pub sound_effects: LibraryEntry,
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone)]
pub enum LibraryEntry {
    Category { // 3544,Aquatic Sounds,1,1,0,0;
        id: u32,
        name: String,
        parent: u32,
        children: Vec<LibraryEntry>,
        enabled: bool,
    },
    Sound { // 10728,Background Ambience Loop 01,0,10642,96677,699;
        id: u32,
        name: String,
        parent: u32,
        bytes: i64,
        duration: i64, // in centiseconds
        enabled: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

impl LibraryEntry {
    pub fn id(&self) -> u32 {
        match self {
            | LibraryEntry::Category { id, .. }
            | LibraryEntry::Sound { id, .. } => *id,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            | LibraryEntry::Category { name, .. }
            | LibraryEntry::Sound { name, .. } => name,
        }
    }
    pub fn pretty_name(&self) -> String {
        if self.is_favourite() {
            format!("{FAVOURITES_CHARACTER} {}", self.name())
        } else {
            self.name().to_string()
        }
    }
    pub fn is_category(&self) -> bool {
        matches!(self, LibraryEntry::Category { .. })
    }
    #[allow(unused)]
    pub fn is_sound(&self) -> bool {
        matches!(self, LibraryEntry::Sound { .. })
    }
    pub fn parent(&self) -> u32 {
        match self {
            | LibraryEntry::Category { parent, .. }
            | LibraryEntry::Sound { parent, .. } => *parent,
        }
    }
    pub fn bytes(&self) -> i64 {
        match self {
            LibraryEntry::Sound { bytes, .. } => *bytes,
            LibraryEntry::Category { .. } => 0,
        }
    }
    pub fn duration(&self) -> i64 {
        match self {
            LibraryEntry::Sound { duration, .. } => *duration,
            LibraryEntry::Category { .. } => 0,
        }
    }
    pub fn push_entry(&mut self, entry: LibraryEntry) {
        if let LibraryEntry::Category { children, .. } = self {
            children.push(entry);
        }
    }
    #[allow(unused)]
    pub fn children(&self) -> Option<&Vec<LibraryEntry>> {
        if let LibraryEntry::Category { children, .. } = self {
            Some(children)
        } else {
            None
        }
    }
    pub fn is_enabled(&self) -> bool {
        match self {
            | LibraryEntry::Category { enabled, .. }
            | LibraryEntry::Sound { enabled, .. } => *enabled,
        }
    }
    pub fn set_enabled(&mut self, new_enabled: bool) {
        let ( LibraryEntry::Category { ref mut enabled, .. }
            | LibraryEntry::Sound { ref mut enabled, .. }) = self;
        *enabled = new_enabled;
    }
    pub fn get_string(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.id(),
            self.name(),
            self.is_category() as u8,
            self.parent(),
            self.bytes(),
            self.duration(),
        )
    }
    pub fn parse_string(string: &str) -> Self {
        let mut entries: Vec<LibraryEntry> = string.split(';').filter_map(|line| {
            let segments = line.split(',').collect::<Vec<&str>>();

            if segments.len() != 6 { return None }

            match segments[2] {
                "0" => Some(LibraryEntry::Sound {
                    id: segments[0].parse().unwrap(),
                    name: segments[1].to_string(),
                    parent: segments[3].parse().unwrap(),
                    bytes: segments[4].parse().unwrap(),
                    duration: segments[5].parse().unwrap(),
                    enabled: true,
                }),
                "1" => Some(LibraryEntry::Category {
                    id: segments[0].parse().unwrap(),
                    name: segments[1].to_string(),
                    parent: segments[3].parse().unwrap(),
                    children: vec![],
                    enabled: true,
                }),
                _ => None
            }
        })
        .collect::<Vec<_>>();

        let mut library_map: HashMap<u32, (&mut LibraryEntry, NodeId)> = HashMap::with_capacity(entries.len());
        let mut library_tree = TreeBuilder::new().with_capacity(entries.len()).with_root(entries[0].id()).build();

        let root_id = entries[0].id();

        for entry in &mut entries {
            if entry.id() != root_id {
                let mut parent_id = library_tree.get_mut((library_map.get(&entry.parent()).unwrap()).1).unwrap();
                let entry_id: slab_tree::NodeMut<'_, u32> = parent_id.append(entry.id());
                library_map.insert(entry.id(), (entry, entry_id.node_id()));
            } else {
                library_map.insert(entry.id(), (entry, library_tree.root_id().unwrap()));
            }
        }

        fn recurse(tree: &NodeRef<'_, u32>, map: &mut HashMap<u32, (&mut LibraryEntry, NodeId)>) {
            for child in tree.children() {
                recurse(&child, map);
            }
            if let Some(parent) = tree.parent() {
                let current_entry = map.get(tree.data()).unwrap().0.clone();
                let parent_entry = map.get_mut(parent.data()).unwrap();
                parent_entry.0.push_entry(current_entry.clone())
            }
        }

        recurse(&library_tree.root().unwrap(), &mut library_map);

        let root = library_map.get(&root_id).unwrap();

        root.0.clone()
    }
    pub fn filename(&self) -> String {
        format!("s{}.ogg", self.id())
    }
    pub fn path(&self) -> PathBuf {
        GD_FOLDER.join(self.filename())
    }
    pub fn download(&self) -> Option<Vec<u8>> {
        if self.is_category() { return None }

        let path = self.path();

        let sfx_data;
        
        if let Some(data) = LOCAL_SFX_LIBRARY.lock().get(&self.id()) {
            return Some(data.clone())
        }
        else if path.exists() {
            sfx_data = fs::read(path).unwrap();
        }
        else if let Some(data) = download_sfx(CDN_URL, self) {
            sfx_data = data;
        }
        else {
            return None
        }
        
        LOCAL_SFX_LIBRARY.lock().insert(self.id(), sfx_data.clone());
        Some(sfx_data)
    }
    pub fn download_and_store(&self) {
        if self.exists() { return }
        if let Some(content) = self.download() {
            fs::write(self.path(), content).unwrap();
            add_file_to_stats(self.id());
        }
    }
    pub fn delete(&self) {
        let _ = fs::remove_file(self.path());
        remove_file_from_stats(self.id());
    }
    pub fn exists(&self) -> bool {
        self.path().exists()
    }
    pub fn is_favourite(&self) -> bool {
        has_favourite(self.id())
    }
    #[allow(unused)]
    pub fn get_all_children(&self) -> Vec<&LibraryEntry> {
        match self {
            LibraryEntry::Sound { .. } => vec![self],
            LibraryEntry::Category { children, .. } => {
                children.iter().flat_map(|child| child.get_all_children()).collect()
            }
        }
    }
}

impl Credit {
    pub fn parse_string(string: &str) -> Vec<Self> {
        string.split(';')
            .filter_map(|c| {
                let data = c.split(',').collect::<Vec<&str>>();
                (data.len() == 2).then(|| Credit {
                    name: data[0].to_string(),
                    link: data[1].to_string(),
                })
            })
            .collect()
    }
}

impl Library {
    pub fn parse_string(string: &str) -> Self {
        let (sound_effects, credits) = string.split_once('|').unwrap_or((string, ""));

        Library {
            sound_effects: LibraryEntry::parse_string(sound_effects),
            credits: Credit::parse_string(credits),
        }
    }
}

pub fn parse_library(data: &[u8]) -> Library {
    let data: Vec<u8> = full_decode(data);
    let string = std::str::from_utf8(&data).unwrap();
    Library::parse_string(string)
}

pub fn update_unlisted_sfx(library: &LibraryEntry) {
    // let entries = library.get_all_children();
    // let library_ids = entries.iter()
    //     .map(|entry| entry.id())
    //     .collect::<Vec<_>>();

    let _ = library;

    thread::spawn(move || {
        // TODO merge this with stats::add_existing_sfx_files

        // for id in library_ids {
        //     all_ids.remove(&id);
        // }

        // *UNLISTED_SFX.lock() = all_ids;
    });
}
