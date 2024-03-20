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

    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
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
}
