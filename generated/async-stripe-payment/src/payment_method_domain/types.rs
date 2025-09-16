/// A payment method domain represents a web domain that you have registered with Stripe.
/// Stripe Elements use registered payment method domains to control where certain payment methods are shown.
///
/// Related guide: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_domains/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDomain {
    pub amazon_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    pub apple_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The domain name that this payment method domain object represents.
    pub domain_name: String,
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub enabled: bool,
    pub google_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodDomainId,
    pub klarna: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    pub link: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub paypal: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
}
#[doc(hidden)]
pub struct PaymentMethodDomainBuilder {
    amazon_pay: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    apple_pay: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    created: Option<stripe_types::Timestamp>,
    domain_name: Option<String>,
    enabled: Option<bool>,
    google_pay: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    id: Option<stripe_payment::PaymentMethodDomainId>,
    klarna: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    link: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
    livemode: Option<bool>,
    paypal: Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus>,
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

    impl Deserialize for PaymentMethodDomain {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDomain>,
        builder: PaymentMethodDomainBuilder,
    }

    impl Visitor for Place<PaymentMethodDomain> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDomainBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDomainBuilder {
        type Out = PaymentMethodDomain;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "created" => Deserialize::begin(&mut self.created),
                "domain_name" => Deserialize::begin(&mut self.domain_name),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "id" => Deserialize::begin(&mut self.id),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "link" => Deserialize::begin(&mut self.link),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "paypal" => Deserialize::begin(&mut self.paypal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amazon_pay: Deserialize::default(),
                apple_pay: Deserialize::default(),
                created: Deserialize::default(),
                domain_name: Deserialize::default(),
                enabled: Deserialize::default(),
                google_pay: Deserialize::default(),
                id: Deserialize::default(),
                klarna: Deserialize::default(),
                link: Deserialize::default(),
                livemode: Deserialize::default(),
                paypal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amazon_pay),
                Some(apple_pay),
                Some(created),
                Some(domain_name),
                Some(enabled),
                Some(google_pay),
                Some(id),
                Some(klarna),
                Some(link),
                Some(livemode),
                Some(paypal),
            ) = (
                self.amazon_pay.take(),
                self.apple_pay.take(),
                self.created,
                self.domain_name.take(),
                self.enabled,
                self.google_pay.take(),
                self.id.take(),
                self.klarna.take(),
                self.link.take(),
                self.livemode,
                self.paypal.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amazon_pay,
                apple_pay,
                created,
                domain_name,
                enabled,
                google_pay,
                id,
                klarna,
                link,
                livemode,
                paypal,
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

    impl ObjectDeser for PaymentMethodDomain {
        type Builder = PaymentMethodDomainBuilder;
    }

    impl FromValueOpt for PaymentMethodDomain {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDomainBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "apple_pay" => b.apple_pay = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "domain_name" => b.domain_name = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "google_pay" => b.google_pay = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDomain {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentMethodDomain", 12)?;
        s.serialize_field("amazon_pay", &self.amazon_pay)?;
        s.serialize_field("apple_pay", &self.apple_pay)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("domain_name", &self.domain_name)?;
        s.serialize_field("enabled", &self.enabled)?;
        s.serialize_field("google_pay", &self.google_pay)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("klarna", &self.klarna)?;
        s.serialize_field("link", &self.link)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("paypal", &self.paypal)?;

        s.serialize_field("object", "payment_method_domain")?;
        s.end()
    }
}
impl stripe_types::Object for PaymentMethodDomain {
    type Id = stripe_payment::PaymentMethodDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentMethodDomainId);
