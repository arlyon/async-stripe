#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Shipping {
    pub address: Option<stripe_shared::Address>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    pub carrier: Option<String>,
    /// Recipient name.
    pub name: Option<String>,
    /// Recipient phone (including extension).
    pub phone: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub tracking_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Shipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Shipping").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingBuilder {
    address: Option<Option<stripe_shared::Address>>,
    carrier: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tracking_number: Option<Option<String>>,
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

    impl Deserialize for Shipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Shipping>,
        builder: ShippingBuilder,
    }

    impl Visitor for Place<Shipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingBuilder {
                    address: Deserialize::default(),
                    carrier: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                    tracking_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "carrier" => Deserialize::begin(&mut self.builder.carrier),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "tracking_number" => Deserialize::begin(&mut self.builder.tracking_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address), Some(carrier), Some(name), Some(phone), Some(tracking_number)) = (
                self.builder.address.take(),
                self.builder.carrier.take(),
                self.builder.name.take(),
                self.builder.phone.take(),
                self.builder.tracking_number.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(Shipping { address, carrier, name, phone, tracking_number });
            Ok(())
        }
    }
};
