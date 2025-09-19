#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3 {
    /// List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission.
    pub required_actions: Vec<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>,
    /// Visa Compelling Evidence 3.0 eligibility status.
    pub status: DisputeEnhancedEligibilityVisaCompellingEvidence3Status,
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3Builder {
    required_actions: Option<Vec<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>>,
    status: Option<DisputeEnhancedEligibilityVisaCompellingEvidence3Status>,
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

    impl Deserialize for DisputeEnhancedEligibilityVisaCompellingEvidence3 {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEligibilityVisaCompellingEvidence3>,
        builder: DisputeEnhancedEligibilityVisaCompellingEvidence3Builder,
    }

    impl Visitor for Place<DisputeEnhancedEligibilityVisaCompellingEvidence3> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEligibilityVisaCompellingEvidence3Builder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEligibilityVisaCompellingEvidence3Builder {
        type Out = DisputeEnhancedEligibilityVisaCompellingEvidence3;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "required_actions" => Deserialize::begin(&mut self.required_actions),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { required_actions: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(required_actions), Some(status)) =
                (self.required_actions.take(), self.status)
            else {
                return None;
            };
            Some(Self::Out { required_actions, status })
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

    impl ObjectDeser for DisputeEnhancedEligibilityVisaCompellingEvidence3 {
        type Builder = DisputeEnhancedEligibilityVisaCompellingEvidence3Builder;
    }

    impl FromValueOpt for DisputeEnhancedEligibilityVisaCompellingEvidence3 {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEligibilityVisaCompellingEvidence3Builder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "required_actions" => b.required_actions = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    MissingCustomerIdentifiers,
    MissingDisputedTransactionDescription,
    MissingMerchandiseOrServices,
    MissingPriorUndisputedTransactionDescription,
    MissingPriorUndisputedTransactions,
}
impl DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    pub fn as_str(self) -> &'static str {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::*;
        match self {
            MissingCustomerIdentifiers => "missing_customer_identifiers",
            MissingDisputedTransactionDescription => "missing_disputed_transaction_description",
            MissingMerchandiseOrServices => "missing_merchandise_or_services",
            MissingPriorUndisputedTransactionDescription => {
                "missing_prior_undisputed_transaction_description"
            }
            MissingPriorUndisputedTransactions => "missing_prior_undisputed_transactions",
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::*;
        match s {
            "missing_customer_identifiers" => Ok(MissingCustomerIdentifiers),
            "missing_disputed_transaction_description" => Ok(MissingDisputedTransactionDescription),
            "missing_merchandise_or_services" => Ok(MissingMerchandiseOrServices),
            "missing_prior_undisputed_transaction_description" => {
                Ok(MissingPriorUndisputedTransactionDescription)
            }
            "missing_prior_undisputed_transactions" => Ok(MissingPriorUndisputedTransactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions"))
    }
}
/// Visa Compelling Evidence 3.0 eligibility status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    NotQualified,
    Qualified,
    RequiresAction,
}
impl DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    pub fn as_str(self) -> &'static str {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3Status::*;
        match self {
            NotQualified => "not_qualified",
            Qualified => "qualified",
            RequiresAction => "requires_action",
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3Status::*;
        match s {
            "not_qualified" => Ok(NotQualified),
            "qualified" => Ok(Qualified),
            "requires_action" => Ok(RequiresAction),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<DisputeEnhancedEligibilityVisaCompellingEvidence3Status>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeEnhancedEligibilityVisaCompellingEvidence3Status::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputeEnhancedEligibilityVisaCompellingEvidence3Status);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for DisputeEnhancedEligibilityVisaCompellingEvidence3Status",
            )
        })
    }
}
