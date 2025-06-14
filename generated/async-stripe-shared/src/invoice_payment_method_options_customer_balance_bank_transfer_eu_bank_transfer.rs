#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry,
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder {
    country: Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
        builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder {
        type Out = InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country),) = (self.country,) else {
                return None;
            };
            Some(Self::Out { country })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
        type Builder = InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The desired country code of the bank account information.
/// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    Be,
    De,
    Es,
    Fr,
    Ie,
    Nl,
}
impl InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::*;
        match self {
            Be => "BE",
            De => "DE",
            Es => "ES",
            Fr => "FR",
            Ie => "IE",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::*;
        match s {
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "FR" => Ok(Fr),
            "IE" => Ok(Ie),
            "NL" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::from_str(
                s,
            )
            .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry"))
    }
}
