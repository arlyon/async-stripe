#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3 {
    /// List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission.
    pub required_actions: Vec<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>,
    /// Visa Compelling Evidence 3.0 eligibility status.
    pub status: DisputeEnhancedEligibilityVisaCompellingEvidence3Status,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEligibilityVisaCompellingEvidence3").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3Builder {
    required_actions: Option<Vec<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>>,
    status: Option<DisputeEnhancedEligibilityVisaCompellingEvidence3Status>,
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
                builder: DisputeEnhancedEligibilityVisaCompellingEvidence3Builder {
                    required_actions: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "required_actions" => Deserialize::begin(&mut self.builder.required_actions),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(required_actions), Some(status)) =
                (self.builder.required_actions.take(), self.builder.status.take())
            else {
                return Ok(());
            };
            *self.out = Some(DisputeEnhancedEligibilityVisaCompellingEvidence3 {
                required_actions,
                status,
            });
            Ok(())
        }
    }
};
/// List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    MissingCustomerIdentifiers,
    MissingDisputedTransactionDescription,
    MissingMerchandiseOrServices,
    MissingPriorUndisputedTransactionDescription,
    MissingPriorUndisputedTransactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    pub fn as_str(&self) -> &str {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::*;
        match self {
            MissingCustomerIdentifiers => "missing_customer_identifiers",
            MissingDisputedTransactionDescription => "missing_disputed_transaction_description",
            MissingMerchandiseOrServices => "missing_merchandise_or_services",
            MissingPriorUndisputedTransactionDescription => {
                "missing_prior_undisputed_transaction_description"
            }
            MissingPriorUndisputedTransactions => "missing_prior_undisputed_transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Visa Compelling Evidence 3.0 eligibility status.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    NotQualified,
    Qualified,
    RequiresAction,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    pub fn as_str(&self) -> &str {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3Status::*;
        match self {
            NotQualified => "not_qualified",
            Qualified => "qualified",
            RequiresAction => "requires_action",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEnhancedEligibilityVisaCompellingEvidence3Status::*;
        match s {
            "not_qualified" => Ok(NotQualified),
            "qualified" => Ok(Qualified),
            "requires_action" => Ok(RequiresAction),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeEnhancedEligibilityVisaCompellingEvidence3Status"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputeEnhancedEligibilityVisaCompellingEvidence3Status))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<DisputeEnhancedEligibilityVisaCompellingEvidence3Status>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeEnhancedEligibilityVisaCompellingEvidence3Status::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
