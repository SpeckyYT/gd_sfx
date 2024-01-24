use std::time::Duration;
use crate::EntryId;

#[derive(Debug)]
pub struct Credit {
    pub id: EntryId,
    pub name: String,
    pub url: String,
    pub yt_channel_id: String,
}

#[derive(Debug)]
pub struct Song {
    pub id: EntryId,
    pub name: String,
    pub credit_id: EntryId,
    pub bytes: i64,
    pub duration: Duration,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Tag {
    pub id: EntryId,
    pub name: String,
}
