#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceRenderingPdf {
    /// Page size of invoice pdf.
    /// Options include a4, letter, and auto.
    /// If set to auto, page size will be switched to a4 or letter based on customer locale.
    pub page_size: Option<InvoiceRenderingPdfPageSize>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceRenderingPdf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceRenderingPdf").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceRenderingPdfBuilder {
    page_size: Option<Option<InvoiceRenderingPdfPageSize>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for InvoiceRenderingPdf {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceRenderingPdf>,
        builder: InvoiceRenderingPdfBuilder,
    }

    impl Visitor for Place<InvoiceRenderingPdf> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceRenderingPdfBuilder { page_size: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "page_size" => Deserialize::begin(&mut self.builder.page_size),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(page_size),) = (self.builder.page_size.take(),) else {
                return Ok(());
            };
            *self.out = Some(InvoiceRenderingPdf { page_size });
            Ok(())
        }
    }
};
/// Page size of invoice pdf.
/// Options include a4, letter, and auto.
/// If set to auto, page size will be switched to a4 or letter based on customer locale.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceRenderingPdfPageSize {
    pub fn as_str(&self) -> &str {
        use InvoiceRenderingPdfPageSize::*;
        match self {
            A4 => "a4",
            Auto => "auto",
            Letter => "letter",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceRenderingPdfPageSize {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceRenderingPdfPageSize::*;
        match s {
            "a4" => Ok(A4),
            "auto" => Ok(Auto),
            "letter" => Ok(Letter),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceRenderingPdfPageSize"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceRenderingPdfPageSize)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceRenderingPdfPageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InvoiceRenderingPdfPageSize {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InvoiceRenderingPdfPageSize> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceRenderingPdfPageSize::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceRenderingPdfPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
