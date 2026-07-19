#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationPendingRequest {
    /// The additional amount Stripe will hold if the authorization is approved, in the card's [currency](https://docs.stripe.com/api#issuing_authorization_object-pending-request-currency) and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount_details: Option<stripe_shared::IssuingAuthorizationAmountDetails>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// If set `true`, you may provide [amount](https://docs.stripe.com/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    pub is_amount_controllable: bool,
    /// The amount the merchant is requesting to be authorized in the `merchant_currency`.
    /// The amount is in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub merchant_amount: i64,
    /// The local currency the merchant is requesting to authorize.
    pub merchant_currency: stripe_types::Currency,
    /// The card network's estimate of the likelihood that an authorization is fraudulent.
    /// Takes on values between 1 and 99.
    pub network_risk_score: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationPendingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationPendingRequest").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingAuthorizationPendingRequestBuilder {
                    amount: Deserialize::default(),
                    amount_details: Deserialize::default(),
                    currency: Deserialize::default(),
                    is_amount_controllable: Deserialize::default(),
                    merchant_amount: Deserialize::default(),
                    merchant_currency: Deserialize::default(),
                    network_risk_score: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_details" => Deserialize::begin(&mut self.builder.amount_details),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "is_amount_controllable" => {
                    Deserialize::begin(&mut self.builder.is_amount_controllable)
                }
                "merchant_amount" => Deserialize::begin(&mut self.builder.merchant_amount),
                "merchant_currency" => Deserialize::begin(&mut self.builder.merchant_currency),
                "network_risk_score" => Deserialize::begin(&mut self.builder.network_risk_score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_details),
                Some(currency),
                Some(is_amount_controllable),
                Some(merchant_amount),
                Some(merchant_currency),
                Some(network_risk_score),
            ) = (
                self.builder.amount,
                self.builder.amount_details,
                self.builder.currency.take(),
                self.builder.is_amount_controllable,
                self.builder.merchant_amount,
                self.builder.merchant_currency.take(),
                self.builder.network_risk_score,
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationPendingRequest {
                amount,
                amount_details,
                currency,
                is_amount_controllable,
                merchant_amount,
                merchant_currency,
                network_risk_score,
            });
            Ok(())
        }
    }
};
