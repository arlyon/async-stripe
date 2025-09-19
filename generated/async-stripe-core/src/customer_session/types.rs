/// A Customer Session allows you to grant Stripe's frontend SDKs (like Stripe.js) client-side access
/// control over a Customer.
///
/// Related guides: [Customer Session with the Payment Element](/payments/accept-a-payment-deferred?platform=web&type=payment#save-payment-methods),.
/// [Customer Session with the Pricing Table](/payments/checkout/pricing-table#customer-session),
/// [Customer Session with the Buy Button](/payment-links/buy-button#pass-an-existing-customer).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSession {
    /// The client secret of this Customer Session.
    /// Used on the client to set up secure access to the given `customer`.
    ///
    /// The client secret can be used to provide access to `customer` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the relevant customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: String,
    pub components: Option<stripe_core::CustomerSessionResourceComponents>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The Customer the Customer Session was created for.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// The timestamp at which this Customer Session will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[doc(hidden)]
pub struct CustomerSessionBuilder {
    client_secret: Option<String>,
    components: Option<Option<stripe_core::CustomerSessionResourceComponents>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    expires_at: Option<stripe_types::Timestamp>,
    livemode: Option<bool>,
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

    impl Deserialize for CustomerSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSession>,
        builder: CustomerSessionBuilder,
    }

    impl Visitor for Place<CustomerSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerSessionBuilder {
        type Out = CustomerSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "components" => Deserialize::begin(&mut self.components),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                client_secret: Deserialize::default(),
                components: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                expires_at: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(client_secret),
                Some(components),
                Some(created),
                Some(customer),
                Some(expires_at),
                Some(livemode),
            ) = (
                self.client_secret.take(),
                self.components.take(),
                self.created,
                self.customer.take(),
                self.expires_at,
                self.livemode,
            )
            else {
                return None;
            };
            Some(Self::Out { client_secret, components, created, customer, expires_at, livemode })
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

    impl ObjectDeser for CustomerSession {
        type Builder = CustomerSessionBuilder;
    }

    impl FromValueOpt for CustomerSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "components" => b.components = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CustomerSession", 7)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("components", &self.components)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "customer_session")?;
        s.end()
    }
}
