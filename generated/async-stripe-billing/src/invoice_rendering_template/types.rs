/// Invoice Rendering Templates are used to configure how invoices are rendered on surfaces like the PDF.
/// Invoice Rendering Templates.
/// can be created from within the Dashboard, and they can be used over the API when creating invoices.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceRenderingTemplate {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_billing::InvoiceRenderingTemplateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A brief description of the template, hidden from customers
    pub nickname: Option<String>,
    /// The status of the template, one of `active` or `archived`.
    pub status: stripe_billing::InvoiceRenderingTemplateStatus,
    /// Version of this template; version increases by one when an update on the template changes any field that controls invoice rendering.
    pub version: i64,
}
#[doc(hidden)]
pub struct InvoiceRenderingTemplateBuilder {
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_billing::InvoiceRenderingTemplateId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    nickname: Option<Option<String>>,
    status: Option<stripe_billing::InvoiceRenderingTemplateStatus>,
    version: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceRenderingTemplate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceRenderingTemplate>,
        builder: InvoiceRenderingTemplateBuilder,
    }

    impl Visitor for Place<InvoiceRenderingTemplate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceRenderingTemplateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceRenderingTemplateBuilder {
        type Out = InvoiceRenderingTemplate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "nickname" => Deserialize::begin(&mut self.nickname),
                "status" => Deserialize::begin(&mut self.status),
                "version" => Deserialize::begin(&mut self.version),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                nickname: Deserialize::default(),
                status: Deserialize::default(),
                version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(nickname),
                Some(status),
                Some(version),
            ) = (
                self.created,
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.nickname.take(),
                self.status,
                self.version,
            )
            else {
                return None;
            };
            Some(Self::Out { created, id, livemode, metadata, nickname, status, version })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InvoiceRenderingTemplate {
        type Builder = InvoiceRenderingTemplateBuilder;
    }

    impl FromValueOpt for InvoiceRenderingTemplate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceRenderingTemplateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "nickname" => b.nickname = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "version" => b.version = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceRenderingTemplate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("InvoiceRenderingTemplate", 8)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("nickname", &self.nickname)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("version", &self.version)?;

        s.serialize_field("object", "invoice_rendering_template")?;
        s.end()
    }
}
impl stripe_types::Object for InvoiceRenderingTemplate {
    type Id = stripe_billing::InvoiceRenderingTemplateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(InvoiceRenderingTemplateId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceRenderingTemplateStatus {
    Active,
    Archived,
}
impl InvoiceRenderingTemplateStatus {
    pub fn as_str(self) -> &'static str {
        use InvoiceRenderingTemplateStatus::*;
        match self {
            Active => "active",
            Archived => "archived",
        }
    }
}

impl std::str::FromStr for InvoiceRenderingTemplateStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceRenderingTemplateStatus::*;
        match s {
            "active" => Ok(Active),
            "archived" => Ok(Archived),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceRenderingTemplateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceRenderingTemplateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceRenderingTemplateStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceRenderingTemplateStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceRenderingTemplateStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceRenderingTemplateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InvoiceRenderingTemplateStatus")
        })
    }
}
