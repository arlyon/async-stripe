use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Derives {
    copy: bool,
    default: bool,
    eq: bool,
    serialize: bool,
    deserialize: bool,
    miniserde_deserialize: bool,
}

impl Derives {
    pub fn new() -> Self {
        Self {
            copy: false,
            default: false,
            eq: false,
            serialize: false,
            deserialize: false,
            miniserde_deserialize: false,
        }
    }

    pub fn deser() -> Self {
        Self {
            copy: false,
            default: false,
            eq: false,
            serialize: true,
            deserialize: true,
            miniserde_deserialize: true,
        }
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

    pub fn miniserde_deserialize(mut self, miniserde_deserialize: bool) -> Self {
        self.miniserde_deserialize = miniserde_deserialize;
        self
    }

    pub fn derives_deserialize(&self) -> bool {
        self.deserialize
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
    let _ = write!(out, ")]");

    if derives.deserialize {
        let _ = write!(out, r#"#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]"#);
    }
    if derives.miniserde_deserialize {
        let _ = write!(out, r#"#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]"#);
    }

    out
}
