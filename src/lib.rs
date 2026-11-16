// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ike: ike component model.
//!
//! Real implementation: ike_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IkeItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IkeError {
    EmptyIkeItemName,
    DuplicateIkeItem(String),
    EmptyEventSubject,
}

impl fmt::Display for IkeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIkeItemName => write!(f, "ike_item name must not be empty"),
            Self::DuplicateIkeItem(id) => write!(f, "duplicate ike_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IkeError {}

pub struct Ike {
    pub name: String,
    ike_items: BTreeMap<String, IkeItem>,
    events: BTreeMap<String, Event>,
}

impl Ike {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ike_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ike_item(&mut self, item: IkeItem) -> Result<(), IkeError> {
        if item.name.trim().is_empty() {
            return Err(IkeError::EmptyIkeItemName);
        }
        let id = item.id.clone();
        if self.ike_items.contains_key(&id) {
            return Err(IkeError::DuplicateIkeItem(id.clone()));
        }
        self.ike_items.insert(id, item);
        Ok(())
    }

    pub fn ike_item_count(&self) -> usize {
        self.ike_items.len()
    }

    pub fn get_ike_item(&self, id: &str) -> Option<&IkeItem> {
        self.ike_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IkeError> {
        if event.subject.trim().is_empty() {
            return Err(IkeError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ike_item(id: &str, name: &str) -> IkeItem {
        IkeItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Ike::new("Ike");
        assert_eq!(m.name, "Ike");
        assert_eq!(m.ike_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ike_item() {
        let mut m = Ike::new("Ike");
        m.add_ike_item(sample_ike_item("a1", "Alpha")).unwrap();
        assert_eq!(m.ike_item_count(), 1);
        assert!(m.get_ike_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Ike::new("Ike");
        assert!(m.add_ike_item(sample_ike_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Ike::new("Ike");
        m.add_ike_item(sample_ike_item("x", "One")).unwrap();
        assert!(m.add_ike_item(sample_ike_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Ike::new("Ike");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
