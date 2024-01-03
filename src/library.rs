use eframe::epaint::ahash::{HashMap, HashMapExt};
use slab_tree::{TreeBuilder, NodeId, NodeRef};

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
        durations: i64, // in centiseconds
    },
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
    pub fn parent(&self) -> i64 {
        match self {
            LibraryEntry::Category { parent, .. } => *parent,
            LibraryEntry::Sound { parent, .. } => *parent,
        }
    }
    pub fn push_entry(&mut self, entry: LibraryEntry) {
        if let LibraryEntry::Category { children, .. } = self {
            children.push(entry);
        }
    }
    pub fn is_category(&self) -> bool {
        match self {
            LibraryEntry::Category { .. } => true,
            LibraryEntry::Sound { .. } => false,
        }
    }
    pub fn is_sound(&self) -> bool {
        match self {
            LibraryEntry::Category { .. } => false,
            LibraryEntry::Sound { .. } => true,
        }
    }
    pub fn from_string(string: &str) -> Self {
        let mut entries: Vec<LibraryEntry> = string.split(";").filter_map(|line| {
            let segments = line.split(",").collect::<Vec<&str>>();

            if segments.len() != 6 { return None }

            match segments[2] {
                "0" => Some(LibraryEntry::Sound {
                    id: segments[0].parse().unwrap(),
                    name: segments[1].to_string(),
                    parent: segments[3].parse().unwrap(),
                    bytes: segments[4].parse().unwrap(),
                    durations: segments[5].parse().unwrap(),
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
                let current_entry = map.get(&tree.data()).unwrap().0.clone();
                let parent_entry = map.get_mut(&parent.data()).unwrap();
                parent_entry.0.push_entry(current_entry.clone())
            }
        }

        recurse(&library_tree.root().unwrap(), &mut library_map);

        let root = library_map.get(&root_id).unwrap();
        
        println!("done!");

        root.0.clone()
    }
}
