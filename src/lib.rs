// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-tool: tool component model.
//!
//! Real implementation: tool_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolItem {
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
pub enum ToolError {
    EmptyToolItemName,
    DuplicateToolItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyToolItemName => write!(f, "tool_item name must not be empty"),
            Self::DuplicateToolItem(id) => write!(f, "duplicate tool_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ToolError {}

pub struct Tool {
    pub name: String,
    tool_items: BTreeMap<String, ToolItem>,
    events: BTreeMap<String, Event>,
}

impl Tool {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tool_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_tool_item(&mut self, item: ToolItem) -> Result<(), ToolError> {
        if item.name.trim().is_empty() {
            return Err(ToolError::EmptyToolItemName);
        }
        let id = item.id.clone();
        if self.tool_items.contains_key(&id) {
            return Err(ToolError::DuplicateToolItem(id.clone()));
        }
        self.tool_items.insert(id, item);
        Ok(())
    }

    pub fn tool_item_count(&self) -> usize {
        self.tool_items.len()
    }

    pub fn get_tool_item(&self, id: &str) -> Option<&ToolItem> {
        self.tool_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ToolError> {
        if event.subject.trim().is_empty() {
            return Err(ToolError::EmptyEventSubject);
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

    fn sample_tool_item(id: &str, name: &str) -> ToolItem {
        ToolItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Tool::new("Tool");
        assert_eq!(m.name, "Tool");
        assert_eq!(m.tool_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_tool_item() {
        let mut m = Tool::new("Tool");
        m.add_tool_item(sample_tool_item("a1", "Alpha")).unwrap();
        assert_eq!(m.tool_item_count(), 1);
        assert!(m.get_tool_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Tool::new("Tool");
        assert!(m.add_tool_item(sample_tool_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Tool::new("Tool");
        m.add_tool_item(sample_tool_item("x", "One")).unwrap();
        assert!(m.add_tool_item(sample_tool_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Tool::new("Tool");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
