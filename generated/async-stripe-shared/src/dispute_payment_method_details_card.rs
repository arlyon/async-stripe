#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsCard {
    /// Card brand.
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,
    /// The card network's specific dispute reason code, which maps to one of Stripe's primary dispute categories to simplify response guidance.
    /// The [Network code map](https://stripe.com/docs/disputes/categories#network-code-map) lists all available dispute reason codes by network.
    pub network_reason_code: Option<String>,
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsCardBuilder {
    brand: Option<String>,
    network_reason_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DisputePaymentMethodDetailsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsCard>,
        builder: DisputePaymentMethodDetailsCardBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsCardBuilder {
        type Out = DisputePaymentMethodDetailsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "network_reason_code" => Deserialize::begin(&mut self.network_reason_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brand: Deserialize::default(), network_reason_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                brand: self.brand.take()?,
                network_reason_code: self.network_reason_code.take()?,
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

    impl ObjectDeser for DisputePaymentMethodDetailsCard {
        type Builder = DisputePaymentMethodDetailsCardBuilder;
    }

    impl FromValueOpt for DisputePaymentMethodDetailsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputePaymentMethodDetailsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = Some(FromValueOpt::from_value(v)?),
                    "network_reason_code" => {
                        b.network_reason_code = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
