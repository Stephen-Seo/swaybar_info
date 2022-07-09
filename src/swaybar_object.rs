use std::fmt::Display;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaybarArray {
    objects: Vec<SwaybarObject>,
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
    pub fn new() -> Self {
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
            name: None,
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
    }

    pub fn from_string(string: String) -> Self {
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
            name: None,
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None,
        }
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
            name: None,
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
        }
    }

    pub fn push_object(&mut self, object: SwaybarObject) {
        self.objects.push(object);
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
