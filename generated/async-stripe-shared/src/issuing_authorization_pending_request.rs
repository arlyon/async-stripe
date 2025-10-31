#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationPendingRequest {
    /// The additional amount Stripe will hold if the authorization is approved, in the card's [currency](https://stripe.com/docs/api#issuing_authorization_object-pending-request-currency) and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If set `true`, you may provide [amount](https://stripe.com/docs/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub is_amount_controllable: bool,
    /// The amount the merchant is requesting to be authorized in the `merchant_currency`.
    /// The amount is in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The local currency the merchant is requesting to authorize.
    pub merchant_currency: stripe_types::Currency,
    /// The card network's estimate of the likelihood that an authorization is fraudulent.
    /// Takes on values between 1 and 99.
    pub network_risk_score: Option<i64>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationPendingRequestBuilder {
    amount: Option<i64>,
    amount_details: Option<Option<stripe_shared::IssuingAuthorizationAmountDetails>>,
    currency: Option<stripe_types::Currency>,
    is_amount_controllable: Option<bool>,
    merchant_amount: Option<i64>,
    merchant_currency: Option<stripe_types::Currency>,
    network_risk_score: Option<Option<i64>>,
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

    impl Deserialize for IssuingAuthorizationPendingRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationPendingRequest>,
        builder: IssuingAuthorizationPendingRequestBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationPendingRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationPendingRequestBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationPendingRequestBuilder {
        type Out = IssuingAuthorizationPendingRequest;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_details" => Deserialize::begin(&mut self.amount_details),
                "currency" => Deserialize::begin(&mut self.currency),
                "is_amount_controllable" => Deserialize::begin(&mut self.is_amount_controllable),
                "merchant_amount" => Deserialize::begin(&mut self.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.merchant_currency),
                "network_risk_score" => Deserialize::begin(&mut self.network_risk_score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_details: Deserialize::default(),
                currency: Deserialize::default(),
                is_amount_controllable: Deserialize::default(),
                merchant_amount: Deserialize::default(),
                merchant_currency: Deserialize::default(),
                network_risk_score: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_details),
                Some(currency),
                Some(is_amount_controllable),
                Some(merchant_amount),
                Some(merchant_currency),
                Some(network_risk_score),
            ) = (
                self.amount,
                self.amount_details,
                self.currency.take(),
                self.is_amount_controllable,
                self.merchant_amount,
                self.merchant_currency.take(),
                self.network_risk_score,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_details,
                currency,
                is_amount_controllable,
                merchant_amount,
                merchant_currency,
                network_risk_score,
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

    impl ObjectDeser for IssuingAuthorizationPendingRequest {
        type Builder = IssuingAuthorizationPendingRequestBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationPendingRequest {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationPendingRequestBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_details" => b.amount_details = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "is_amount_controllable" => {
                        b.is_amount_controllable = FromValueOpt::from_value(v)
                    }
                    "merchant_amount" => b.merchant_amount = FromValueOpt::from_value(v),
                    "merchant_currency" => b.merchant_currency = FromValueOpt::from_value(v),
                    "network_risk_score" => b.network_risk_score = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
