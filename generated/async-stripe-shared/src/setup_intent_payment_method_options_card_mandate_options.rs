#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments, specified in the presentment currency.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: stripe_shared::AmountType,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: String,
    /// Start date of the mandate or subscription. Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported. Possible values are `india`.
    pub supported_types: Option<Vec<stripe_shared::SupportedTypes>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCardMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsCardMandateOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder {
    amount: Option<i64>,
    amount_type: Option<stripe_shared::AmountType>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    end_date: Option<Option<stripe_types::Timestamp>>,
    interval: Option<stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval>,
    interval_count: Option<Option<u64>>,
    reference: Option<String>,
    start_date: Option<stripe_types::Timestamp>,
    supported_types: Option<Option<Vec<stripe_shared::SupportedTypes>>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsCardMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsCardMandateOptions>,
        builder: SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsCardMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder {
        type Out = SetupIntentPaymentMethodOptionsCardMandateOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                "reference" => Deserialize::begin(&mut self.reference),
                "start_date" => Deserialize::begin(&mut self.start_date),
                "supported_types" => Deserialize::begin(&mut self.supported_types),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: None,
                amount_type: None,
                currency: None,
                description: Some(None),
                end_date: Some(None),
                interval: None,
                interval_count: Some(None),
                reference: None,
                start_date: None,
                supported_types: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_type),
                Some(currency),
                Some(description),
                Some(end_date),
                Some(interval),
                Some(interval_count),
                Some(reference),
                Some(start_date),
                Some(supported_types),
            ) = (
                self.amount,
                self.amount_type.take(),
                self.currency.take(),
                self.description.take(),
                self.end_date,
                self.interval.take(),
                self.interval_count,
                self.reference.take(),
                self.start_date,
                self.supported_types.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_type,
                currency,
                description,
                end_date,
                interval,
                interval_count,
                reference,
                start_date,
                supported_types,
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsCardMandateOptions {
        type Builder = SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsCardMandateOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsCardMandateOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    "supported_types" => b.supported_types = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
pub type SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType = stripe_shared::AmountType;
/// Specifies payment frequency. One of `day`, `week`, `month`, `year`, or `sporadic`.
pub type SetupIntentPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_shared::PaymentMethodOptionsCardMandateOptionsInterval;
/// Specifies the type of mandates supported. Possible values are `india`.
pub type SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes =
    stripe_shared::SupportedTypes;
