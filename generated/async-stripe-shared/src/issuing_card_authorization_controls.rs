#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardAuthorizationControls {
    /// Array of card presence statuses from which authorizations will be allowed.
    /// Possible options are `present`, `not_present`.
    /// All other statuses will be blocked.
    /// Cannot be set with `blocked_card_presences`.
    /// Provide an empty value to unset this control.
    pub allowed_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    pub allowed_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be allowed.
    /// Authorizations from merchants in all other countries will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `blocked_merchant_countries`.
    /// Provide an empty value to unset this control.
    pub allowed_merchant_countries: Option<Vec<String>>,
    /// Array of card presence statuses from which authorizations will be declined.
    /// Possible options are `present`, `not_present`.
    /// Cannot be set with `allowed_card_presences`.
    /// Provide an empty value to unset this control.
    pub blocked_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    pub blocked_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `allowed_merchant_countries`.
    /// Provide an empty value to unset this control.
    pub blocked_merchant_countries: Option<Vec<String>>,
    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    pub spending_limits: Option<Vec<stripe_shared::IssuingCardSpendingLimit>>,
    /// Currency of the amounts within `spending_limits`. Always the same as the currency of the card.
    pub spending_limits_currency: Option<stripe_types::Currency>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardAuthorizationControls {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardAuthorizationControls").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardAuthorizationControlsBuilder {
    allowed_card_presences: Option<Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>>,
    allowed_categories: Option<Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>>,
    allowed_merchant_countries: Option<Option<Vec<String>>>,
    blocked_card_presences: Option<Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>>,
    blocked_categories: Option<Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>>,
    blocked_merchant_countries: Option<Option<Vec<String>>>,
    spending_limits: Option<Option<Vec<stripe_shared::IssuingCardSpendingLimit>>>,
    spending_limits_currency: Option<Option<stripe_types::Currency>>,
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

    impl Deserialize for IssuingCardAuthorizationControls {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardAuthorizationControls>,
        builder: IssuingCardAuthorizationControlsBuilder,
    }

    impl Visitor for Place<IssuingCardAuthorizationControls> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardAuthorizationControlsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardAuthorizationControlsBuilder {
        type Out = IssuingCardAuthorizationControls;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allowed_card_presences" => Deserialize::begin(&mut self.allowed_card_presences),
                "allowed_categories" => Deserialize::begin(&mut self.allowed_categories),
                "allowed_merchant_countries" => {
                    Deserialize::begin(&mut self.allowed_merchant_countries)
                }
                "blocked_card_presences" => Deserialize::begin(&mut self.blocked_card_presences),
                "blocked_categories" => Deserialize::begin(&mut self.blocked_categories),
                "blocked_merchant_countries" => {
                    Deserialize::begin(&mut self.blocked_merchant_countries)
                }
                "spending_limits" => Deserialize::begin(&mut self.spending_limits),
                "spending_limits_currency" => {
                    Deserialize::begin(&mut self.spending_limits_currency)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                allowed_card_presences: Some(None),
                allowed_categories: Some(None),
                allowed_merchant_countries: Some(None),
                blocked_card_presences: Some(None),
                blocked_categories: Some(None),
                blocked_merchant_countries: Some(None),
                spending_limits: Some(None),
                spending_limits_currency: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(allowed_card_presences),
                Some(allowed_categories),
                Some(allowed_merchant_countries),
                Some(blocked_card_presences),
                Some(blocked_categories),
                Some(blocked_merchant_countries),
                Some(spending_limits),
                Some(spending_limits_currency),
            ) = (
                self.allowed_card_presences.take(),
                self.allowed_categories.take(),
                self.allowed_merchant_countries.take(),
                self.blocked_card_presences.take(),
                self.blocked_categories.take(),
                self.blocked_merchant_countries.take(),
                self.spending_limits.take(),
                self.spending_limits_currency.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                allowed_card_presences,
                allowed_categories,
                allowed_merchant_countries,
                blocked_card_presences,
                blocked_categories,
                blocked_merchant_countries,
                spending_limits,
                spending_limits_currency,
            })
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

    impl ObjectDeser for IssuingCardAuthorizationControls {
        type Builder = IssuingCardAuthorizationControlsBuilder;
    }

    impl FromValueOpt for IssuingCardAuthorizationControls {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardAuthorizationControlsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allowed_card_presences" => {
                        b.allowed_card_presences = FromValueOpt::from_value(v)
                    }
                    "allowed_categories" => b.allowed_categories = FromValueOpt::from_value(v),
                    "allowed_merchant_countries" => {
                        b.allowed_merchant_countries = FromValueOpt::from_value(v)
                    }
                    "blocked_card_presences" => {
                        b.blocked_card_presences = FromValueOpt::from_value(v)
                    }
                    "blocked_categories" => b.blocked_categories = FromValueOpt::from_value(v),
                    "blocked_merchant_countries" => {
                        b.blocked_merchant_countries = FromValueOpt::from_value(v)
                    }
                    "spending_limits" => b.spending_limits = FromValueOpt::from_value(v),
                    "spending_limits_currency" => {
                        b.spending_limits_currency = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Array of card presence statuses from which authorizations will be allowed.
/// Possible options are `present`, `not_present`.
/// All other statuses will be blocked.
/// Cannot be set with `blocked_card_presences`.
/// Provide an empty value to unset this control.
pub type IssuingCardAuthorizationControlsAllowedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
pub type IssuingCardAuthorizationControlsAllowedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of card presence statuses from which authorizations will be declined.
/// Possible options are `present`, `not_present`.
/// Cannot be set with `allowed_card_presences`.
/// Provide an empty value to unset this control.
pub type IssuingCardAuthorizationControlsBlockedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
pub type IssuingCardAuthorizationControlsBlockedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
