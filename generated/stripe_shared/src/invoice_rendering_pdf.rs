#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceRenderingPdf {
    /// Page size of invoice pdf.
    /// Options include a4, letter, and auto.
    /// If set to auto, page size will be switched to a4 or letter based on customer locale.
    pub page_size: Option<InvoiceRenderingPdfPageSize>,
}
#[doc(hidden)]
pub struct InvoiceRenderingPdfBuilder {
    page_size: Option<Option<InvoiceRenderingPdfPageSize>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: InvoiceRenderingPdfBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceRenderingPdfBuilder {
        type Out = InvoiceRenderingPdf;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "page_size" => Deserialize::begin(&mut self.page_size),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { page_size: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { page_size: self.page_size? })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InvoiceRenderingPdf {
        type Builder = InvoiceRenderingPdfBuilder;
    }

    impl FromValueOpt for InvoiceRenderingPdf {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceRenderingPdfBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "page_size" => b.page_size = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Page size of invoice pdf.
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
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceRenderingPdfPageSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceRenderingPdfPageSize {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceRenderingPdfPageSize> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceRenderingPdfPageSize::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceRenderingPdfPageSize);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceRenderingPdfPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceRenderingPdfPageSize"))
    }
}
