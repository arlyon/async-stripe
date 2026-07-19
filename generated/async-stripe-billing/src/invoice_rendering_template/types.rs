/// Invoice Rendering Templates are used to configure how invoices are rendered on surfaces like the PDF.
/// Invoice Rendering Templates.
/// can be created from within the Dashboard, and they can be used over the API when creating invoices.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceRenderingTemplate {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_billing::InvoiceRenderingTemplateId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A brief description of the template, hidden from customers
    pub nickname: Option<String>,
    /// The status of the template, one of `active` or `archived`.
    pub status: stripe_billing::InvoiceRenderingTemplateStatus,
    /// Version of this template; version increases by one when an update on the template changes any field that controls invoice rendering.
    pub version: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceRenderingTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceRenderingTemplate").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: InvoiceRenderingTemplateBuilder {
                    created: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    nickname: Deserialize::default(),
                    status: Deserialize::default(),
                    version: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "nickname" => Deserialize::begin(&mut self.builder.nickname),
                "status" => Deserialize::begin(&mut self.builder.status),
                "version" => Deserialize::begin(&mut self.builder.version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(nickname),
                Some(status),
                Some(version),
            ) = (
                self.builder.created,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.nickname.take(),
                self.builder.status.take(),
                self.builder.version,
            )
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceRenderingTemplate {
                created,
                id,
                livemode,
                metadata,
                nickname,
                status,
                version,
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceRenderingTemplateStatus {
    Active,
    Archived,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceRenderingTemplateStatus {
    pub fn as_str(&self) -> &str {
        use InvoiceRenderingTemplateStatus::*;
        match self {
            Active => "active",
            Archived => "archived",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceRenderingTemplateStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceRenderingTemplateStatus::*;
        match s {
            "active" => Ok(Active),
            "archived" => Ok(Archived),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceRenderingTemplateStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceRenderingTemplateStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for InvoiceRenderingTemplateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InvoiceRenderingTemplateStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceRenderingTemplateStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceRenderingTemplateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
