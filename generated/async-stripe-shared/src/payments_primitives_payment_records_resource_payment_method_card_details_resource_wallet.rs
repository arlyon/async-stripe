#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet {
pub apple_pay: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
pub dynamic_last4: Option<String>,
pub google_pay: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceGooglePay>,
        /// The type of the card wallet, one of `apple_pay` or `google_pay`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: String,

}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder {
    apple_pay: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceApplePay>>,
dynamic_last4: Option<Option<String>>,
google_pay: Option<Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletResourceGooglePay>>,
type_: Option<String>,

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
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "dynamic_last4" => Deserialize::begin(&mut self.dynamic_last4),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                apple_pay: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                google_pay: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(apple_pay), Some(dynamic_last4), Some(google_pay), Some(type_)) = (
                self.apple_pay.take(),
                self.dynamic_last4.take(),
                self.google_pay,
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { apple_pay, dynamic_last4, google_pay, type_ })
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

    impl ObjectDeser
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet
    {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder;
    }

    impl FromValueOpt
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "apple_pay" => b.apple_pay = FromValueOpt::from_value(v),
                    "dynamic_last4" => b.dynamic_last4 = FromValueOpt::from_value(v),
                    "google_pay" => b.google_pay = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
