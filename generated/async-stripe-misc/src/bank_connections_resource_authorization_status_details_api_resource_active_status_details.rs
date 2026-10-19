#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails {
    /// The action (if any) to proactively relink the Authorization.
    pub action: stripe_shared::Action,
    /// When the Authorization is expected to become inactive, if applicable.
    pub expected_deactivation_date: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder {
    action: Option<stripe_shared::Action>,
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

    impl Deserialize
        for BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails,
        >,
        builder:
            BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder,
    }

    impl Visitor
        for Place<BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder
    {
        type Out = BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "expected_deactivation_date" => {
                    Deserialize::begin(&mut self.expected_deactivation_date)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { action: None, expected_deactivation_date: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(action), Some(expected_deactivation_date)) =
                (self.action.take(), self.expected_deactivation_date)
            else {
                return None;
            };
            Some(Self::Out { action, expected_deactivation_date })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser
        for BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails
    {
        type Builder =
            BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder;
    }

    impl FromValueOpt
        for BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "action" => b.action = FromValueOpt::from_value(v),
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
/// The action (if any) to proactively relink the Authorization.
pub type BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetailsAction =
    stripe_shared::Action;
