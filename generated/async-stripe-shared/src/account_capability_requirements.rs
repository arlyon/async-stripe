#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilityRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account.
    /// These fields may disable the capability sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be collected to keep the capability enabled.
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,
    /// If the capability is disabled, this string describes why.
    /// Can be `requirements.fields_needed`, `pending.onboarding`, `pending.review`, `rejected.fraud`, `rejected.other`, `platform_paused`, `action_required.requested_capabilities`, `rejected.inactivty`, or `rejected.unsupported_business`.
    ///
    /// `rejected.unsupported_business` means that the account's business is not supported by the capability.
    /// For example, payment methods may restrict the businesses they support in their terms of service, such as in [Afterpay Clearpay's terms of service](/afterpay-clearpay/legal#restricted-businesses).
    ///
    /// `rejected.inactivity` means that the capability has been paused for inactivity.
    /// This disabled reason currently only applies to the Issuing capability.
    /// See [Issuing: Managing Inactive Connects](https://support.stripe.com/questions/issuing-managing-inactive-connect-accounts) for more details.
    ///
    /// If you believe that a rejection is in error, please contact support at <https://support.stripe.com/contact/> for assistance.
    pub disabled_reason: Option<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by `current_deadline`.
    /// These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,
    /// Fields that might become required depending on the results of verification or review.
    /// It's an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    /// Fields might appear in `eventually_due`, `currently_due`, or `past_due` and in `pending_verification` if verification fails but another verification is still pending.
    pub pending_verification: Vec<String>,
}
#[doc(hidden)]
pub struct AccountCapabilityRequirementsBuilder {
    alternatives: Option<Option<Vec<stripe_shared::AccountRequirementsAlternative>>>,
    current_deadline: Option<Option<stripe_types::Timestamp>>,
    currently_due: Option<Vec<String>>,
    disabled_reason: Option<Option<String>>,
    errors: Option<Vec<stripe_shared::AccountRequirementsError>>,
    eventually_due: Option<Vec<String>>,
    past_due: Option<Vec<String>>,
    pending_verification: Option<Vec<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountCapabilityRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilityRequirements>,
        builder: AccountCapabilityRequirementsBuilder,
    }

    impl Visitor for Place<AccountCapabilityRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCapabilityRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCapabilityRequirementsBuilder {
        type Out = AccountCapabilityRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alternatives" => Deserialize::begin(&mut self.alternatives),
                "current_deadline" => Deserialize::begin(&mut self.current_deadline),
                "currently_due" => Deserialize::begin(&mut self.currently_due),
                "disabled_reason" => Deserialize::begin(&mut self.disabled_reason),
                "errors" => Deserialize::begin(&mut self.errors),
                "eventually_due" => Deserialize::begin(&mut self.eventually_due),
                "past_due" => Deserialize::begin(&mut self.past_due),
                "pending_verification" => Deserialize::begin(&mut self.pending_verification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alternatives: Deserialize::default(),
                current_deadline: Deserialize::default(),
                currently_due: Deserialize::default(),
                disabled_reason: Deserialize::default(),
                errors: Deserialize::default(),
                eventually_due: Deserialize::default(),
                past_due: Deserialize::default(),
                pending_verification: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                alternatives: self.alternatives.take()?,
                current_deadline: self.current_deadline?,
                currently_due: self.currently_due.take()?,
                disabled_reason: self.disabled_reason.take()?,
                errors: self.errors.take()?,
                eventually_due: self.eventually_due.take()?,
                past_due: self.past_due.take()?,
                pending_verification: self.pending_verification.take()?,
            })
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

    impl ObjectDeser for AccountCapabilityRequirements {
        type Builder = AccountCapabilityRequirementsBuilder;
    }

    impl FromValueOpt for AccountCapabilityRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountCapabilityRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alternatives" => b.alternatives = Some(FromValueOpt::from_value(v)?),
                    "current_deadline" => b.current_deadline = Some(FromValueOpt::from_value(v)?),
                    "currently_due" => b.currently_due = Some(FromValueOpt::from_value(v)?),
                    "disabled_reason" => b.disabled_reason = Some(FromValueOpt::from_value(v)?),
                    "errors" => b.errors = Some(FromValueOpt::from_value(v)?),
                    "eventually_due" => b.eventually_due = Some(FromValueOpt::from_value(v)?),
                    "past_due" => b.past_due = Some(FromValueOpt::from_value(v)?),
                    "pending_verification" => {
                        b.pending_verification = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
