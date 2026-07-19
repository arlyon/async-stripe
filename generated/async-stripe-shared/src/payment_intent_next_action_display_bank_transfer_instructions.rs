#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    /// The remaining amount that needs to be transferred to complete the payment.
    pub amount_remaining: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// A list of financial addresses that can be used to fund the customer balance
    pub financial_addresses:
        Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>,
    /// A link to a hosted page that guides your customer through completing the transfer.
    pub hosted_instructions_url: Option<String>,
    /// A string identifying this payment.
    /// Instruct your customer to include this code in the reference or memo field of their bank transfer.
    pub reference: Option<String>,
    /// Type of bank transfer
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentIntentNextActionDisplayBankTransferInstructionsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionDisplayBankTransferInstructions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
    amount_remaining: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    financial_addresses:
        Option<Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>>,
    hosted_instructions_url: Option<Option<String>>,
    reference: Option<Option<String>>,
    type_: Option<PaymentIntentNextActionDisplayBankTransferInstructionsType>,
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

    impl Deserialize for PaymentIntentNextActionDisplayBankTransferInstructions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayBankTransferInstructions>,
        builder: PaymentIntentNextActionDisplayBankTransferInstructionsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayBankTransferInstructions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionDisplayBankTransferInstructionsBuilder {
                    amount_remaining: Deserialize::default(),
                    currency: Deserialize::default(),
                    financial_addresses: Deserialize::default(),
                    hosted_instructions_url: Deserialize::default(),
                    reference: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_remaining" => Deserialize::begin(&mut self.builder.amount_remaining),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "financial_addresses" => Deserialize::begin(&mut self.builder.financial_addresses),
                "hosted_instructions_url" => {
                    Deserialize::begin(&mut self.builder.hosted_instructions_url)
                }
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount_remaining),
                Some(currency),
                Some(financial_addresses),
                Some(hosted_instructions_url),
                Some(reference),
                Some(type_),
            ) = (
                self.builder.amount_remaining,
                self.builder.currency.take(),
                self.builder.financial_addresses.take(),
                self.builder.hosted_instructions_url.take(),
                self.builder.reference.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionDisplayBankTransferInstructions {
                amount_remaining,
                currency,
                financial_addresses,
                hosted_instructions_url,
                reference,
                type_,
            });
            Ok(())
        }
    }
};
/// Type of bank transfer
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentNextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentNextActionDisplayBankTransferInstructionsType {
    pub fn as_str(&self) -> &str {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentNextActionDisplayBankTransferInstructionsType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentNextActionDisplayBankTransferInstructionsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentIntentNextActionDisplayBankTransferInstructionsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentIntentNextActionDisplayBankTransferInstructionsType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentNextActionDisplayBankTransferInstructionsType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
