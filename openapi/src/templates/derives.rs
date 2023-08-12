#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Derives {
    pub copy: bool,
    pub debug: bool,
    pub default: bool,
    pub eq: bool,
    pub serialize: bool,
    pub deserialize: bool,
}

impl Derives {
    pub const fn new() -> Self {
        Self {
            debug: true,
            copy: false,
            default: false,
            eq: false,
            serialize: false,
            deserialize: false,
        }
    }

    pub const fn new_deser() -> Self {
        Self { serialize: true, deserialize: true, ..Self::new() }
    }

    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    pub fn copy(&mut self, copy: bool) -> &mut Self {
        self.copy = copy;
        self
    }

    pub fn default(&mut self, default: bool) -> &mut Self {
        self.default = default;
        self
    }

    pub fn eq(&mut self, eq: bool) -> &mut Self {
        self.eq = eq;
        self
    }

    pub fn serialize(&mut self, serialize: bool) -> &mut Self {
        self.serialize = serialize;
        self
    }

    pub fn deserialize(&mut self, deserialize: bool) -> &mut Self {
        self.deserialize = deserialize;
        self
    }
}
