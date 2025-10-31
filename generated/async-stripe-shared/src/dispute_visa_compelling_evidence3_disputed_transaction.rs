#[derive(Clone, Debug)]
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
                builder: DisputeVisaCompellingEvidence3DisputedTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeVisaCompellingEvidence3DisputedTransactionBuilder {
        type Out = DisputeVisaCompellingEvidence3DisputedTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_account_id" => Deserialize::begin(&mut self.customer_account_id),
                "customer_device_fingerprint" => {
                    Deserialize::begin(&mut self.customer_device_fingerprint)
                }
                "customer_device_id" => Deserialize::begin(&mut self.customer_device_id),
                "customer_email_address" => Deserialize::begin(&mut self.customer_email_address),
                "customer_purchase_ip" => Deserialize::begin(&mut self.customer_purchase_ip),
                "merchandise_or_services" => Deserialize::begin(&mut self.merchandise_or_services),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "shipping_address" => Deserialize::begin(&mut self.shipping_address),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer_account_id: Deserialize::default(),
                customer_device_fingerprint: Deserialize::default(),
                customer_device_id: Deserialize::default(),
                customer_email_address: Deserialize::default(),
                customer_purchase_ip: Deserialize::default(),
                merchandise_or_services: Deserialize::default(),
                product_description: Deserialize::default(),
                shipping_address: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.customer_account_id.take(),
                self.customer_device_fingerprint.take(),
                self.customer_device_id.take(),
                self.customer_email_address.take(),
                self.customer_purchase_ip.take(),
                self.merchandise_or_services,
                self.product_description.take(),
                self.shipping_address.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                customer_account_id,
                customer_device_fingerprint,
                customer_device_id,
                customer_email_address,
                customer_purchase_ip,
                merchandise_or_services,
                product_description,
                shipping_address,
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

    impl ObjectDeser for DisputeVisaCompellingEvidence3DisputedTransaction {
        type Builder = DisputeVisaCompellingEvidence3DisputedTransactionBuilder;
    }

    impl FromValueOpt for DisputeVisaCompellingEvidence3DisputedTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeVisaCompellingEvidence3DisputedTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_account_id" => b.customer_account_id = FromValueOpt::from_value(v),
                    "customer_device_fingerprint" => {
                        b.customer_device_fingerprint = FromValueOpt::from_value(v)
                    }
                    "customer_device_id" => b.customer_device_id = FromValueOpt::from_value(v),
                    "customer_email_address" => {
                        b.customer_email_address = FromValueOpt::from_value(v)
                    }
                    "customer_purchase_ip" => b.customer_purchase_ip = FromValueOpt::from_value(v),
                    "merchandise_or_services" => {
                        b.merchandise_or_services = FromValueOpt::from_value(v)
                    }
                    "product_description" => b.product_description = FromValueOpt::from_value(v),
                    "shipping_address" => b.shipping_address = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Categorization of disputed payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    Merchandise,
    Services,
}
impl DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    pub fn as_str(self) -> &'static str {
        use DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match self {
            Merchandise => "merchandise",
            Services => "services",
        }
    }
}

impl std::str::FromStr for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match s {
            "merchandise" => Ok(Merchandise),
            "services" => Ok(Services),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices"))
    }
}
