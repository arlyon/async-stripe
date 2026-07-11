#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails {
    /// The action (if any) to proactively relink the Account.
    pub action: BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction,
    /// The underlying cause of the Account becoming inactive.
    pub cause: BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause,
    /// When the Account is expected to become inactive, if applicable.
    pub expected_deactivation_date: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder {
    action: Option<BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction>,
    cause: Option<BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause>,
    expected_deactivation_date: Option<stripe_types::Timestamp>,
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

    impl Deserialize for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails,
        >,
        builder: BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder
    {
        type Out = BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "cause" => Deserialize::begin(&mut self.cause),
                "expected_deactivation_date" => {
                    Deserialize::begin(&mut self.expected_deactivation_date)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { action: None, cause: None, expected_deactivation_date: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(action), Some(cause), Some(expected_deactivation_date)) =
                (self.action.take(), self.cause.take(), self.expected_deactivation_date)
            else {
                return None;
            };
            Some(Self::Out { action, cause, expected_deactivation_date })
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

    impl ObjectDeser for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails {
        type Builder =
            BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "action" => b.action = FromValueOpt::from_value(v),
                    "cause" => b.cause = FromValueOpt::from_value(v),
                    "expected_deactivation_date" => {
                        b.expected_deactivation_date = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The action (if any) to proactively relink the Account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction {
    None,
    RelinkRequired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction::*;
        match self {
            None => "none",
            RelinkRequired => "relink_required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction::*;
        match s {
            "none" => Ok(None),
            "relink_required" => Ok(RelinkRequired),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsAction
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The underlying cause of the Account becoming inactive.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause {
    AccessExpired,
    InstitutionRequirement,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause::*;
        match self {
            AccessExpired => "access_expired",
            InstitutionRequirement => "institution_requirement",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause::*;
        match s {
            "access_expired" => Ok(AccessExpired),
            "institution_requirement" => Ok(InstitutionRequirement),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetailsCause
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
