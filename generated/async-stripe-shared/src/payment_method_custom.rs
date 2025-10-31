#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCustom {
    /// Display name of the Dashboard-only CustomPaymentMethodType.
    pub display_name: Option<String>,
    /// Contains information about the Dashboard-only CustomPaymentMethodType logo.
    pub logo: Option<stripe_shared::CustomLogo>,
    /// ID of the Dashboard-only CustomPaymentMethodType. Not expandable.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct PaymentMethodCustomBuilder {
    display_name: Option<Option<String>>,
    logo: Option<Option<stripe_shared::CustomLogo>>,
    type_: Option<String>,
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

    impl Deserialize for PaymentMethodCustom {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCustom>,
        builder: PaymentMethodCustomBuilder,
    }

    impl Visitor for Place<PaymentMethodCustom> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCustomBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCustomBuilder {
        type Out = PaymentMethodCustom;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.display_name),
                "logo" => Deserialize::begin(&mut self.logo),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                display_name: Deserialize::default(),
                logo: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display_name), Some(logo), Some(type_)) =
                (self.display_name.take(), self.logo.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { display_name, logo, type_ })
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

    impl ObjectDeser for PaymentMethodCustom {
        type Builder = PaymentMethodCustomBuilder;
    }

    impl FromValueOpt for PaymentMethodCustom {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCustomBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "logo" => b.logo = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
