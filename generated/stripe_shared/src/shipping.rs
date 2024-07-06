#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct ShippingBuilder {
    address: Option<Option<stripe_shared::Address>>,
    carrier: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tracking_number: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
            Ok(Box::new(Builder { out: &mut self.out, builder: ShippingBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ShippingBuilder {
        type Out = Shipping;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "carrier" => Deserialize::begin(&mut self.carrier),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                "tracking_number" => Deserialize::begin(&mut self.tracking_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                carrier: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
                tracking_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address: self.address.take()?,
                carrier: self.carrier.take()?,
                name: self.name.take()?,
                phone: self.phone.take()?,
                tracking_number: self.tracking_number.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Shipping {
        type Builder = ShippingBuilder;
    }

    impl FromValueOpt for Shipping {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ShippingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "carrier" => b.carrier = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "phone" => b.phone = Some(FromValueOpt::from_value(v)?),
                    "tracking_number" => b.tracking_number = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
