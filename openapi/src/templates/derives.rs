/// Attributes a struct or enum can derive
#[derive(Debug, Copy, Clone)]
pub enum Derives {
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Default,
}

impl Derives {
    pub fn as_str(&self) -> &'static str {
        match self {
            Derives::Clone => "Clone",
            Derives::Debug => "Debug",
            Derives::Serialize => "Serialize",
            Derives::Deserialize => "Deserialize",
            Derives::Default => "Default",
        }
    }
}

pub fn write_derives_line(derives: &[Derives]) -> String {
    let inner = derives.iter().map(Derives::as_str).collect::<Vec<_>>().join(",");
    format!("#[derive({inner})]")
}
