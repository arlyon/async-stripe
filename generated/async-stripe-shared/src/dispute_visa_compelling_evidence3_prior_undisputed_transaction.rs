#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeVisaCompellingEvidence3PriorUndisputedTransaction {
    /// Stripe charge ID for the Visa Compelling Evidence 3.0 eligible prior charge.
    pub charge: String,
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
    /// A description of the product or service that was sold.
    pub product_description: Option<String>,
    /// The address to which a physical product was shipped.
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
    pub shipping_address: Option<stripe_shared::DisputeTransactionShippingAddress>,
}
#[doc(hidden)]
pub struct DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder {
    charge: Option<String>,
    customer_account_id: Option<Option<String>>,
    customer_device_fingerprint: Option<Option<String>>,
    customer_device_id: Option<Option<String>>,
    customer_email_address: Option<Option<String>>,
    customer_purchase_ip: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DisputeVisaCompellingEvidence3PriorUndisputedTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeVisaCompellingEvidence3PriorUndisputedTransaction>,
        builder: DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder,
    }

    impl Visitor for Place<DisputeVisaCompellingEvidence3PriorUndisputedTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder {
        type Out = DisputeVisaCompellingEvidence3PriorUndisputedTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "customer_account_id" => Deserialize::begin(&mut self.customer_account_id),
                "customer_device_fingerprint" => {
                    Deserialize::begin(&mut self.customer_device_fingerprint)
                }
                "customer_device_id" => Deserialize::begin(&mut self.customer_device_id),
                "customer_email_address" => Deserialize::begin(&mut self.customer_email_address),
                "customer_purchase_ip" => Deserialize::begin(&mut self.customer_purchase_ip),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "shipping_address" => Deserialize::begin(&mut self.shipping_address),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge: Deserialize::default(),
                customer_account_id: Deserialize::default(),
                customer_device_fingerprint: Deserialize::default(),
                customer_device_id: Deserialize::default(),
                customer_email_address: Deserialize::default(),
                customer_purchase_ip: Deserialize::default(),
                product_description: Deserialize::default(),
                shipping_address: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(charge),
                Some(customer_account_id),
                Some(customer_device_fingerprint),
                Some(customer_device_id),
                Some(customer_email_address),
                Some(customer_purchase_ip),
                Some(product_description),
                Some(shipping_address),
            ) = (
                self.charge.take(),
                self.customer_account_id.take(),
                self.customer_device_fingerprint.take(),
                self.customer_device_id.take(),
                self.customer_email_address.take(),
                self.customer_purchase_ip.take(),
                self.product_description.take(),
                self.shipping_address.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                charge,
                customer_account_id,
                customer_device_fingerprint,
                customer_device_id,
                customer_email_address,
                customer_purchase_ip,
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

    impl ObjectDeser for DisputeVisaCompellingEvidence3PriorUndisputedTransaction {
        type Builder = DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder;
    }

    impl FromValueOpt for DisputeVisaCompellingEvidence3PriorUndisputedTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                DisputeVisaCompellingEvidence3PriorUndisputedTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "customer_account_id" => b.customer_account_id = FromValueOpt::from_value(v),
                    "customer_device_fingerprint" => {
                        b.customer_device_fingerprint = FromValueOpt::from_value(v)
                    }
                    "customer_device_id" => b.customer_device_id = FromValueOpt::from_value(v),
                    "customer_email_address" => {
                        b.customer_email_address = FromValueOpt::from_value(v)
                    }
                    "customer_purchase_ip" => b.customer_purchase_ip = FromValueOpt::from_value(v),
                    "product_description" => b.product_description = FromValueOpt::from_value(v),
                    "shipping_address" => b.shipping_address = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
