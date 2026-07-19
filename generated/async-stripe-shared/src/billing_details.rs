#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<stripe_shared::Address>,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
    /// Billing phone number (including extension).
    pub phone: Option<String>,
    /// Taxpayer identification number.
    /// Used only for transactions between LATAM buyers and non-LATAM sellers.
    pub tax_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tax_id: Option<Option<String>>,
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

    impl Deserialize for BillingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingDetails>,
        builder: BillingDetailsBuilder,
    }

    impl Visitor for Place<BillingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingDetailsBuilder {
                    address: Deserialize::default(),
                    email: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                    tax_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "email" => Deserialize::begin(&mut self.builder.email),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "tax_id" => Deserialize::begin(&mut self.builder.tax_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address), Some(email), Some(name), Some(phone), Some(tax_id)) = (
                self.builder.address.take(),
                self.builder.email.take(),
                self.builder.name.take(),
                self.builder.phone.take(),
                self.builder.tax_id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingDetails { address, email, name, phone, tax_id });
            Ok(())
        }
    }
};
