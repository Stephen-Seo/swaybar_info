use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaybarHeader {
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cont_signal: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaybarObject {
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<String>,
}

#[derive(Debug)]
pub struct SwaybarArray {
    objects: Vec<SwaybarObject>,
    objects_idx_map: HashMap<String, usize>,
}

impl SwaybarHeader {
    pub fn new() -> Self {
        Self {
            version: 1,
            click_events: None,
            cont_signal: None,
            stop_signal: None,
        }
    }
}

impl SwaybarObject {
    pub fn new(name: String) -> Self {
        Self {
            full_text: String::new(),
            short_text: None,
            color: None,
            background: None,
            border: Some("#ffffffff".into()),
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            min_width: None,
            align: None,
            name: Some(name),
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }

    pub fn from_string(name: String, string: String) -> Self {
        Self {
            full_text: string,
            short_text: None,
            color: None,
            background: None,
            border: Some("#ffffffff".into()),
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            min_width: None,
            align: None,
            name: Some(name),
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }

    pub fn from_error_string(name: String, msg: String) -> Self {
        Self {
            full_text: msg,
            short_text: None,
            color: Some("#ff2222ff".into()),
            background: None,
            border: Some("#ffffffff".into()),
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            min_width: None,
            align: None,
            name: Some(name),
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }

    pub fn update_as_net_down(&mut self, metric: String) {
        self.full_text = metric;
        self.color = Some("#ff8888ff".to_owned());
    }

    pub fn update_as_net_up(&mut self, metric: String) {
        self.full_text = metric;
        self.color = Some("#88ff88ff".to_owned());
    }

    pub fn update_as_date(&mut self) {
        let current_time: DateTime<Local> = Local::now();
        let current_time = current_time.format("%F %r");
        self.full_text = current_time.to_string();
        self.color = None;
    }

    pub fn update_as_generic(&mut self, metric: String, color: Option<String>) {
        self.full_text = metric;
        self.color = color;
    }

    pub fn update_as_error(&mut self, msg: String) {
        self.full_text = msg;
        self.color = Some("#ff2222ff".to_owned());
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for SwaybarObject {
    fn default() -> Self {
        let current_time: DateTime<Local> = Local::now();
        let current_time = current_time.format("%F %r");
        Self {
            full_text: current_time.to_string(),
            short_text: None,
            color: None,
            background: None,
            border: Some("#ffffffff".into()),
            border_top: None,
            border_bottom: None,
            border_left: None,
            border_right: None,
            min_width: None,
            align: None,
            name: Some("current_time".to_owned()),
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }
}

impl SwaybarArray {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            objects_idx_map: HashMap::new(),
        }
    }

    pub fn push_object(&mut self, object: SwaybarObject) {
        self.objects.push(object);
        self.refresh_map();
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn get_by_name(&self, name: &str) -> Option<&SwaybarObject> {
        if let Some(idx) = self.objects_idx_map.get(name) {
            return Some(&self.objects[*idx]);
        }

        None
    }

    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut SwaybarObject> {
        if let Some(idx) = self.objects_idx_map.get(name) {
            return Some(&mut self.objects[*idx]);
        }

        None
    }

    fn refresh_map(&mut self) {
        self.objects_idx_map.clear();
        for (idx, object) in self.objects.iter().enumerate() {
            if let Some(name) = object.get_name() {
                self.objects_idx_map.insert(name.to_owned(), idx);
            }
        }
    }
}

impl Index<usize> for SwaybarArray {
    type Output = SwaybarObject;

    fn index(&self, index: usize) -> &Self::Output {
        self.objects.index(index)
    }
}

impl IndexMut<usize> for SwaybarArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.objects.index_mut(index)
    }
}

impl Display for SwaybarArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = serde_json::to_string(&self.objects)
            .expect("Should be able to serialize SwaybarArray::objects");
        s.push(',');
        f.write_str(&s)
    }
}
