/// A payment method domain represents a web domain that you have registered with Stripe.
/// Stripe Elements use registered payment method domains to control where certain payment methods are shown.
///
/// Related guide: [Payment method domains](https://docs.stripe.com/payments/payment-methods/pmd-registration).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_domains/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub paypal: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDomain").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDomainBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "created" => Deserialize::begin(&mut self.builder.created),
                "domain_name" => Deserialize::begin(&mut self.builder.domain_name),
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "id" => Deserialize::begin(&mut self.builder.id),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "link" => Deserialize::begin(&mut self.builder.link),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.amazon_pay.take(),
                self.builder.apple_pay.take(),
                self.builder.created,
                self.builder.domain_name.take(),
                self.builder.enabled,
                self.builder.google_pay.take(),
                self.builder.id.take(),
                self.builder.klarna.take(),
                self.builder.link.take(),
                self.builder.livemode,
                self.builder.paypal.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDomain {
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
            });
            Ok(())
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
