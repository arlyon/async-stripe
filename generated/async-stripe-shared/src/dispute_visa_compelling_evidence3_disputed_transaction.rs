#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeVisaCompellingEvidence3DisputedTransaction {
    /// User Account ID used to log into business platform. Must be recognizable by the user.
    pub customer_account_id: Option<String>,
    /// Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attributes.
    /// Must be at least 20 characters.
    pub customer_device_fingerprint: Option<String>,
    /// Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Identity [IMEI]).
    /// Must be at least 15 characters.
    pub customer_device_id: Option<String>,
    /// The email address of the customer.
    pub customer_email_address: Option<String>,
    /// The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,
    /// Categorization of disputed payment.
    pub merchandise_or_services:
        Option<DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>,
    /// A description of the product or service that was sold.
    pub product_description: Option<String>,
    /// The address to which a physical product was shipped.
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
    pub shipping_address: Option<stripe_shared::DisputeTransactionShippingAddress>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeVisaCompellingEvidence3DisputedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeVisaCompellingEvidence3DisputedTransaction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeVisaCompellingEvidence3DisputedTransactionBuilder {
    customer_account_id: Option<Option<String>>,
    customer_device_fingerprint: Option<Option<String>>,
    customer_device_id: Option<Option<String>>,
    customer_email_address: Option<Option<String>>,
    customer_purchase_ip: Option<Option<String>>,
    merchandise_or_services:
        Option<Option<DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>>,
    product_description: Option<Option<String>>,
    shipping_address: Option<Option<stripe_shared::DisputeTransactionShippingAddress>>,
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

    impl Deserialize for DisputeVisaCompellingEvidence3DisputedTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeVisaCompellingEvidence3DisputedTransaction>,
        builder: DisputeVisaCompellingEvidence3DisputedTransactionBuilder,
    }

    impl Visitor for Place<DisputeVisaCompellingEvidence3DisputedTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeVisaCompellingEvidence3DisputedTransactionBuilder {
                    customer_account_id: Deserialize::default(),
                    customer_device_fingerprint: Deserialize::default(),
                    customer_device_id: Deserialize::default(),
                    customer_email_address: Deserialize::default(),
                    customer_purchase_ip: Deserialize::default(),
                    merchandise_or_services: Deserialize::default(),
                    product_description: Deserialize::default(),
                    shipping_address: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_account_id" => Deserialize::begin(&mut self.builder.customer_account_id),
                "customer_device_fingerprint" => {
                    Deserialize::begin(&mut self.builder.customer_device_fingerprint)
                }
                "customer_device_id" => Deserialize::begin(&mut self.builder.customer_device_id),
                "customer_email_address" => {
                    Deserialize::begin(&mut self.builder.customer_email_address)
                }
                "customer_purchase_ip" => {
                    Deserialize::begin(&mut self.builder.customer_purchase_ip)
                }
                "merchandise_or_services" => {
                    Deserialize::begin(&mut self.builder.merchandise_or_services)
                }
                "product_description" => Deserialize::begin(&mut self.builder.product_description),
                "shipping_address" => Deserialize::begin(&mut self.builder.shipping_address),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(customer_account_id),
                Some(customer_device_fingerprint),
                Some(customer_device_id),
                Some(customer_email_address),
                Some(customer_purchase_ip),
                Some(merchandise_or_services),
                Some(product_description),
                Some(shipping_address),
            ) = (
                self.builder.customer_account_id.take(),
                self.builder.customer_device_fingerprint.take(),
                self.builder.customer_device_id.take(),
                self.builder.customer_email_address.take(),
                self.builder.customer_purchase_ip.take(),
                self.builder.merchandise_or_services.take(),
                self.builder.product_description.take(),
                self.builder.shipping_address.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(DisputeVisaCompellingEvidence3DisputedTransaction {
                customer_account_id,
                customer_device_fingerprint,
                customer_device_id,
                customer_email_address,
                customer_purchase_ip,
                merchandise_or_services,
                product_description,
                shipping_address,
            });
            Ok(())
        }
    }
};
/// Categorization of disputed payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    Merchandise,
    Services,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    pub fn as_str(&self) -> &str {
        use DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match self {
            Merchandise => "merchandise",
            Services => "services",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "services" => Ok(Services),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
