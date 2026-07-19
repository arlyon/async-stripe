#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCrypto {
    /// The wallet address of the customer.
    pub buyer_address: Option<String>,
    /// The blockchain network that the transaction was sent on.
    pub network: Option<PaymentMethodDetailsCryptoNetwork>,
    /// The token currency that the transaction was sent with.
    pub token_currency: Option<PaymentMethodDetailsCryptoTokenCurrency>,
    /// The blockchain transaction hash of the crypto payment.
    pub transaction_hash: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCrypto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsCrypto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCryptoBuilder {
    buyer_address: Option<Option<String>>,
    network: Option<Option<PaymentMethodDetailsCryptoNetwork>>,
    token_currency: Option<Option<PaymentMethodDetailsCryptoTokenCurrency>>,
    transaction_hash: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsCrypto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCrypto>,
        builder: PaymentMethodDetailsCryptoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCrypto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCryptoBuilder {
                    buyer_address: Deserialize::default(),
                    network: Deserialize::default(),
                    token_currency: Deserialize::default(),
                    transaction_hash: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_address" => Deserialize::begin(&mut self.builder.buyer_address),
                "network" => Deserialize::begin(&mut self.builder.network),
                "token_currency" => Deserialize::begin(&mut self.builder.token_currency),
                "transaction_hash" => Deserialize::begin(&mut self.builder.transaction_hash),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(buyer_address), Some(network), Some(token_currency), Some(transaction_hash)) = (
                self.builder.buyer_address.take(),
                self.builder.network.take(),
                self.builder.token_currency.take(),
                self.builder.transaction_hash.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsCrypto {
                buyer_address,
                network,
                token_currency,
                transaction_hash,
            });
            Ok(())
        }
    }
};
/// The blockchain network that the transaction was sent on.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsCryptoNetwork {
    Base,
    Ethereum,
    Polygon,
    Solana,
    Tempo,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDetailsCryptoNetwork {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDetailsCryptoNetwork::*;
        match self {
            Base => "base",
            Ethereum => "ethereum",
            Polygon => "polygon",
            Solana => "solana",
            Tempo => "tempo",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCryptoNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCryptoNetwork::*;
        match s {
            "base" => Ok(Base),
            "ethereum" => Ok(Ethereum),
            "polygon" => Ok(Polygon),
            "solana" => Ok(Solana),
            "tempo" => Ok(Tempo),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDetailsCryptoNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCryptoNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDetailsCryptoNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCryptoNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDetailsCryptoNetwork)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCryptoNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDetailsCryptoNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCryptoNetwork> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCryptoNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCryptoNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The token currency that the transaction was sent with.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsCryptoTokenCurrency {
    Usdc,
    Usdg,
    Usdp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDetailsCryptoTokenCurrency {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDetailsCryptoTokenCurrency::*;
        match self {
            Usdc => "usdc",
            Usdg => "usdg",
            Usdp => "usdp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCryptoTokenCurrency {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCryptoTokenCurrency::*;
        match s {
            "usdc" => Ok(Usdc),
            "usdg" => Ok(Usdg),
            "usdp" => Ok(Usdp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDetailsCryptoTokenCurrency"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCryptoTokenCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDetailsCryptoTokenCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCryptoTokenCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDetailsCryptoTokenCurrency)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCryptoTokenCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDetailsCryptoTokenCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCryptoTokenCurrency> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCryptoTokenCurrency::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCryptoTokenCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
