use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Derives {
    copy: bool,
    default: bool,
    eq: bool,
    serialize: bool,
    deserialize: bool,
}

impl Derives {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_deser() -> Self {
        Self { copy: false, default: false, eq: false, serialize: true, deserialize: true }
    }

    pub fn copy(mut self, copy: bool) -> Self {
        self.copy = copy;
        self
    }

    pub fn default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    pub fn eq(mut self, eq: bool) -> Self {
        self.eq = eq;
        self
    }

    pub fn serialize(mut self, serialize: bool) -> Self {
        self.serialize = serialize;
        self
    }

    pub fn deserialize(mut self, deserialize: bool) -> Self {
        self.deserialize = deserialize;
        self
    }

    pub fn derives_deserialize(&self) -> bool {
        self.deserialize
    }

    pub fn derives_serialize(&self) -> bool {
        self.serialize
    }

    pub fn derives_default(&self) -> bool {
        self.default
    }
}

pub fn write_derives_line(derives: Derives) -> String {
    let mut out: String = "#[derive(".into();
    if derives.copy {
        let _ = write!(out, "Copy,");
    }
    let _ = write!(out, "Clone, Debug,");
    if derives.default {
        let _ = write!(out, "Default,");
    }
    if derives.eq {
        let _ = write!(out, "Eq, PartialEq,");
    }
    if derives.serialize {
        let _ = write!(out, "serde::Serialize,");
    }
    if derives.deserialize {
        let _ = write!(out, "serde::Deserialize,");
    }
    let _ = write!(out, ")]");
    out
}
