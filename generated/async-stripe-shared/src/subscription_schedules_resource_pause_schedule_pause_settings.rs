#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseSchedulePauseSettings {
    pub bill_for: stripe_shared::SubscriptionSchedulesResourcePauseScheduleBillFor,
    /// Determines how to handle debits and credits when pausing.
    pub invoicing_behavior:
        SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior,
    /// The type of pause settings.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SubscriptionSchedulesResourcePauseSchedulePauseSettingsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePauseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseSchedulePauseSettings")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder {
    bill_for: Option<stripe_shared::SubscriptionSchedulesResourcePauseScheduleBillFor>,
    invoicing_behavior:
        Option<SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior>,
    type_: Option<SubscriptionSchedulesResourcePauseSchedulePauseSettingsType>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseSchedulePauseSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseSchedulePauseSettings>,
        builder: SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseSchedulePauseSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder {
        type Out = SubscriptionSchedulesResourcePauseSchedulePauseSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bill_for" => Deserialize::begin(&mut self.bill_for),
                "invoicing_behavior" => Deserialize::begin(&mut self.invoicing_behavior),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bill_for: None, invoicing_behavior: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bill_for), Some(invoicing_behavior), Some(type_)) =
                (self.bill_for.take(), self.invoicing_behavior.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { bill_for, invoicing_behavior, type_ })
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

    impl ObjectDeser for SubscriptionSchedulesResourcePauseSchedulePauseSettings {
        type Builder = SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseSchedulePauseSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionSchedulesResourcePauseSchedulePauseSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bill_for" => b.bill_for = FromValueOpt::from_value(v),
                    "invoicing_behavior" => b.invoicing_behavior = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines how to handle debits and credits when pausing.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior {
    Invoice,
    PendingInvoiceItem,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior::*;
        match self {
            Invoice => "invoice",
            PendingInvoiceItem => "pending_invoice_item",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior::*;
        match s {
            "invoice" => Ok(Invoice),
            "pending_invoice_item" => Ok(PendingInvoiceItem),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourcePauseSchedulePauseSettingsInvoicingBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of pause settings.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseSchedulePauseSettingsType::*;
        match self {
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseSchedulePauseSettingsType::*;
        match s {
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseSchedulePauseSettingsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionSchedulesResourcePauseSchedulePauseSettingsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseSchedulePauseSettingsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseSchedulePauseSettingsType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseSchedulePauseSettingsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionSchedulesResourcePauseSchedulePauseSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
