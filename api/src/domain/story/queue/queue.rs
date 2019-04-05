use chrono::{ DateTime, Utc };
use crate::domain::Id;
use super::author::{ Author, ActiveAuthor };

// types
#[derive(Debug)]
pub struct Queue {
    pub(super) author_ids:         Vec<Id>,
    pub(super) author_rustle_time: Option<DateTime<Utc>>,
    pub has_new_author:            bool,
    removed_author_index:          Option<usize>
}

// impls
impl Queue {
    pub fn new(author_ids: Vec<Id>, author_rustle_time: Option<DateTime<Utc>>) -> Self {
        Queue {
            author_ids:           author_ids,
            author_rustle_time:   author_rustle_time,
            has_new_author:       false,
            removed_author_index: None,
        }
    }

    // commands
    pub fn join(&mut self, author_id: &Id) {
        self.has_new_author = true;
        self.author_ids.push(author_id.clone());
    }

    pub fn leave(&mut self, author_id: &Id) {
        if self.author_ids.is_empty() {
            warn!("[story] attempted to leave an empty queue");
            return
        }

        let index = match self.author_ids.iter().position(|id| id == author_id) {
            Some(i) => i,
            None    => return warn!("[story] attempted to remove an author that was not in the queue")
        };

        // remove the author
        self.author_ids.remove(index);
        self.removed_author_index = Some(index);
    }

    pub fn rustle_active_author(&mut self, time: DateTime<Utc>) {
        if self.author_ids.is_empty() {
            warn!("[story] attempted to rustle the active author of an empty queue");
            return
        }

        self.author_rustle_time = Some(time)
    }

    // queries
    pub fn active_author(&self) -> Option<ActiveAuthor> {
        if self.author_ids.is_empty() {
            warn!("[story] attempted to get active author of an empty queue");
            return None
        }

        Some(ActiveAuthor::new(
            &self.author_ids[0],
            &self.author_rustle_time
        ))
    }

    pub fn new_author(&self) -> Option<Author> {
        if self.author_ids.is_empty() {
            warn!("[story] attempted to get active author of an empty queue");
            return None
        }

        if !self.has_new_author {
            return None
        }

        self.author_ids.last()
            .map(|id| self.make_author(id, self.author_ids.len() - 1))
    }

    pub fn authors_with_new_positions(&self) -> Vec<Author> {
        let index = match self.removed_author_index {
            Some(index) => index,
            None        => return Vec::new()
        };

        self.author_ids[index..]
            .iter()
            .enumerate()
            .map(|(i, id)| self.make_author(id, i))
            .collect()
    }

    // factories
    fn make_author<'a>(&'a self, id: &'a Id, index: usize) -> Author<'a> {
        if index == 0 {
            Author::active(id, &self.author_rustle_time)
        } else {
            Author::queued(id, index)
        }
    }
}
