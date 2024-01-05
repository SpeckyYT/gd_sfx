use std::{fs, path::PathBuf};

use eframe::epaint::ahash::{HashMap, HashMapExt};
use slab_tree::{TreeBuilder, NodeId, NodeRef};

use crate::{encoding::full_decode, util::{GD_FOLDER, LOCAL_SFX_LIBRARY}, requests::{download_sfx, CDN_URL}, favourites::{has_favourite, FAVOURITES_CHARACTER}, stats::{add_file_to_stats, remove_file_from_stats}};

#[derive(Debug, Clone)]
pub struct Library {
    pub sound_effects: LibraryEntry,
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone)]
pub enum LibraryEntry {
    Category { // 3544,Aquatic Sounds,1,1,0,0;
        id: i64,
        name: String,
        parent: i64,
        children: Vec<LibraryEntry>,
    },
    Sound { // 10728,Background Ambience Loop 01,0,10642,96677,699;
        id: i64,
        name: String,
        parent: i64,
        bytes: i64,
        duration: i64, // in centiseconds
    },
}

#[derive(Debug, Clone)]
pub struct Credit {
    pub name: String,
    pub link: String,
}

impl LibraryEntry {
    pub fn id(&self) -> i64 {
        match self {
            LibraryEntry::Category { id, .. } => *id,
            LibraryEntry::Sound { id, .. } => *id,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            LibraryEntry::Category { name, .. } => name,
            LibraryEntry::Sound { name, .. } => name,
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
        match self {
            LibraryEntry::Category { .. } => true,
            LibraryEntry::Sound { .. } => false,
        }
    }
    #[allow(unused)]
    pub fn is_sound(&self) -> bool {
        match self {
            LibraryEntry::Category { .. } => false,
            LibraryEntry::Sound { .. } => true,
        }
    }
    pub fn parent(&self) -> i64 {
        match self {
            LibraryEntry::Category { parent, .. } => *parent,
            LibraryEntry::Sound { parent, .. } => *parent,
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
                }),
                "1" => Some(LibraryEntry::Category {
                    id: segments[0].parse().unwrap(),
                    name: segments[1].to_string(),
                    parent: segments[3].parse().unwrap(),
                    children: vec![],
                }),
                _ => None
            }
        })
        .collect::<Vec<_>>();

        let mut library_map: HashMap<i64, (&mut LibraryEntry, NodeId)> = HashMap::with_capacity(entries.len());
        let mut library_tree = TreeBuilder::new().with_capacity(entries.len()).with_root(entries[0].id()).build();

        let root_id = entries[0].id();

        for entry in &mut entries {
            if entry.id() != root_id {
                let mut parent_id = library_tree.get_mut((library_map.get(&entry.parent()).unwrap()).1).unwrap();
                let entry_id: slab_tree::NodeMut<'_, i64> = parent_id.append(entry.id());
                library_map.insert(entry.id(), (entry, entry_id.node_id()));
            } else {
                library_map.insert(entry.id(), (entry, library_tree.root_id().unwrap()));
            }
        }

        fn recurse(tree: &NodeRef<'_, i64>, map: &mut HashMap<i64, (&mut LibraryEntry, NodeId)>) {
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
    pub fn download(&self, cdn_url: &str) -> Option<Vec<u8>> {
        if self.is_category() { return None }

        let path = self.path();

        let mut cache_data = true;

        let data =
            if let Some(data) = LOCAL_SFX_LIBRARY.lock().get(&self.id()) {
                cache_data = false;
                data.clone()
            } else if path.exists() {
                fs::read(path).unwrap()
            } else if let Some(data) = download_sfx(cdn_url, self) {
                data
            } else {
                return None
            };
        
        if cache_data {
            LOCAL_SFX_LIBRARY.lock().insert(self.id(), data.clone());
        }

        Some(data)
    }
    pub fn download_and_store(&self) {
        if let Some(content) = self.download(CDN_URL) {
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
}

impl Credit {
    pub fn parse_string(string: &str) -> Vec<Self> {
        string.split(';')
        .filter_map(|c| {
            let data = c.split(',').collect::<Vec<&str>>();
            if data.len() == 2 {
                Some(Credit {
                    name: data[0].to_string(),
                    link: data[1].to_string(),
                })
            } else {
                None
            }
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
