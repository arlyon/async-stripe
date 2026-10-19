#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    pub mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// The network rails used.
    /// See the [docs](https://docs.stripe.com/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    pub network: stripe_shared::TopupResourceUsBankAccountNetwork,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersPaymentMethodDetailsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InboundTransfersPaymentMethodDetailsUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
    account_holder_type: Option<Option<stripe_shared::IssuingCardholderType>>,
    account_type: Option<Option<stripe_shared::PaymentMethodUsBankAccountAccountType>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    network: Option<stripe_shared::TopupResourceUsBankAccountNetwork>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for InboundTransfersPaymentMethodDetailsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InboundTransfersPaymentMethodDetailsUsBankAccount>,
        builder: InboundTransfersPaymentMethodDetailsUsBankAccountBuilder,
    }

    impl Visitor for Place<InboundTransfersPaymentMethodDetailsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InboundTransfersPaymentMethodDetailsUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InboundTransfersPaymentMethodDetailsUsBankAccountBuilder {
        type Out = InboundTransfersPaymentMethodDetailsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "account_type" => Deserialize::begin(&mut self.account_type),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "network" => Deserialize::begin(&mut self.network),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: Some(None),
                account_type: Some(None),
                bank_name: Some(None),
                fingerprint: Some(None),
                last4: Some(None),
                mandate: Some(None),
                network: None,
                routing_number: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder_type),
                Some(account_type),
                Some(bank_name),
                Some(fingerprint),
                Some(last4),
                Some(mandate),
                Some(network),
                Some(routing_number),
            ) = (
                self.account_holder_type.take(),
                self.account_type.take(),
                self.bank_name.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.mandate.take(),
                self.network.take(),
                self.routing_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder_type,
                account_type,
                bank_name,
                fingerprint,
                last4,
                mandate,
                network,
                routing_number,
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

    impl ObjectDeser for InboundTransfersPaymentMethodDetailsUsBankAccount {
        type Builder = InboundTransfersPaymentMethodDetailsUsBankAccountBuilder;
    }

    impl FromValueOpt for InboundTransfersPaymentMethodDetailsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InboundTransfersPaymentMethodDetailsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "account_type" => b.account_type = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "routing_number" => b.routing_number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type: individual or company.
pub type InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType =
    stripe_shared::IssuingCardholderType;
/// Account type: checkings or savings. Defaults to checking if omitted.
pub type InboundTransfersPaymentMethodDetailsUsBankAccountAccountType =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
/// The network rails used.
/// See the [docs](https://docs.stripe.com/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
pub type InboundTransfersPaymentMethodDetailsUsBankAccountNetwork =
    stripe_shared::TopupResourceUsBankAccountNetwork;
