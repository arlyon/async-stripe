#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet",
        )
        .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWalletBuilder { apple_pay: Deserialize::default(),
dynamic_last4: Deserialize::default(),
google_pay: Deserialize::default(),
type_: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "dynamic_last4" => Deserialize::begin(&mut self.builder.dynamic_last4),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(apple_pay), Some(dynamic_last4), Some(google_pay), Some(type_)) = (
                self.builder.apple_pay.take(),
                self.builder.dynamic_last4.take(),
                self.builder.google_pay,
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(
                PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet {
                    apple_pay,
                    dynamic_last4,
                    google_pay,
                    type_,
                },
            );
            Ok(())
        }
    }
};
