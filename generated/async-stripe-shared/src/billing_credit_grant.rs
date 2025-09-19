/// A credit grant is an API resource that documents the allocation of some billing credits to a customer.
///
/// Related guide: [Billing credits](https://docs.stripe.com/billing/subscriptions/usage-based/billing-credits).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrant {
    pub amount: stripe_shared::BillingCreditGrantsResourceAmount,
    pub applicability_config: stripe_shared::BillingCreditGrantsResourceApplicabilityConfig,
    /// The category of this credit grant.
    /// This is for tracking purposes and isn't displayed to the customer.
    pub category: stripe_shared::BillingCreditGrantCategory,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of the customer receiving the billing credits.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// The time when the billing credits become effective-when they're eligible for use.
    pub effective_at: Option<stripe_types::Timestamp>,
    /// The time when the billing credits expire. If not present, the billing credits don't expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::BillingCreditGrantId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A descriptive name shown in dashboard.
    pub name: Option<String>,
    /// The priority for applying this credit grant. The highest priority is 0 and the lowest is 100.
    pub priority: Option<i64>,
    /// ID of the test clock this credit grant belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    /// The time when this credit grant was voided. If not present, the credit grant hasn't been voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct BillingCreditGrantBuilder {
    amount: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
    applicability_config: Option<stripe_shared::BillingCreditGrantsResourceApplicabilityConfig>,
    category: Option<stripe_shared::BillingCreditGrantCategory>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    effective_at: Option<Option<stripe_types::Timestamp>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::BillingCreditGrantId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<Option<String>>,
    priority: Option<Option<i64>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    updated: Option<stripe_types::Timestamp>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for BillingCreditGrant {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrant>,
        builder: BillingCreditGrantBuilder,
    }

    impl Visitor for Place<BillingCreditGrant> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantBuilder {
        type Out = BillingCreditGrant;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "applicability_config" => Deserialize::begin(&mut self.applicability_config),
                "category" => Deserialize::begin(&mut self.category),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "effective_at" => Deserialize::begin(&mut self.effective_at),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                "priority" => Deserialize::begin(&mut self.priority),
                "test_clock" => Deserialize::begin(&mut self.test_clock),
                "updated" => Deserialize::begin(&mut self.updated),
                "voided_at" => Deserialize::begin(&mut self.voided_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                applicability_config: Deserialize::default(),
                category: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                effective_at: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                priority: Deserialize::default(),
                test_clock: Deserialize::default(),
                updated: Deserialize::default(),
                voided_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(applicability_config),
                Some(category),
                Some(created),
                Some(customer),
                Some(effective_at),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(name),
                Some(priority),
                Some(test_clock),
                Some(updated),
                Some(voided_at),
            ) = (
                self.amount.take(),
                self.applicability_config.take(),
                self.category,
                self.created,
                self.customer.take(),
                self.effective_at,
                self.expires_at,
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.name.take(),
                self.priority,
                self.test_clock.take(),
                self.updated,
                self.voided_at,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                applicability_config,
                category,
                created,
                customer,
                effective_at,
                expires_at,
                id,
                livemode,
                metadata,
                name,
                priority,
                test_clock,
                updated,
                voided_at,
            })
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

    impl ObjectDeser for BillingCreditGrant {
        type Builder = BillingCreditGrantBuilder;
    }

    impl FromValueOpt for BillingCreditGrant {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "applicability_config" => b.applicability_config = FromValueOpt::from_value(v),
                    "category" => b.category = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "effective_at" => b.effective_at = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "priority" => b.priority = FromValueOpt::from_value(v),
                    "test_clock" => b.test_clock = FromValueOpt::from_value(v),
                    "updated" => b.updated = FromValueOpt::from_value(v),
                    "voided_at" => b.voided_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditGrant {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingCreditGrant", 16)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("applicability_config", &self.applicability_config)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("effective_at", &self.effective_at)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("priority", &self.priority)?;
        s.serialize_field("test_clock", &self.test_clock)?;
        s.serialize_field("updated", &self.updated)?;
        s.serialize_field("voided_at", &self.voided_at)?;

        s.serialize_field("object", "billing.credit_grant")?;
        s.end()
    }
}
impl stripe_types::Object for BillingCreditGrant {
    type Id = stripe_shared::BillingCreditGrantId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingCreditGrantId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditGrantCategory {
    Paid,
    Promotional,
}
impl BillingCreditGrantCategory {
    pub fn as_str(self) -> &'static str {
        use BillingCreditGrantCategory::*;
        match self {
            Paid => "paid",
            Promotional => "promotional",
        }
    }
}

impl std::str::FromStr for BillingCreditGrantCategory {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantCategory::*;
        match s {
            "paid" => Ok(Paid),
            "promotional" => Ok(Promotional),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditGrantCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditGrantCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BillingCreditGrantCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingCreditGrantCategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditGrantCategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingCreditGrantCategory::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditGrantCategory);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BillingCreditGrantCategory"))
    }
}
