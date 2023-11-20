#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceRenderingPdf {
    /// Page size of invoice pdf.
    ///
    /// Options include a4, letter, and auto.
    /// If set to auto, page size will be switched to a4 or letter based on customer locale.
    pub page_size: Option<InvoiceRenderingPdfPageSize>,
}
/// Page size of invoice pdf.
///
/// Options include a4, letter, and auto.
/// If set to auto, page size will be switched to a4 or letter based on customer locale.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}

impl InvoiceRenderingPdfPageSize {
    pub fn as_str(self) -> &'static str {
        use InvoiceRenderingPdfPageSize::*;
        match self {
            A4 => "a4",
            Auto => "auto",
            Letter => "letter",
        }
    }
}

impl std::str::FromStr for InvoiceRenderingPdfPageSize {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoiceRenderingPdfPageSize {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceRenderingPdfPageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceRenderingPdfPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceRenderingPdfPageSize"))
    }
}
