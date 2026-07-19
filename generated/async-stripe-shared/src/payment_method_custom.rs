#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodCustom").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodCustomBuilder {
                    display_name: Deserialize::default(),
                    logo: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "logo" => Deserialize::begin(&mut self.builder.logo),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(display_name), Some(logo), Some(type_)) = (
                self.builder.display_name.take(),
                self.builder.logo.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodCustom { display_name, logo, type_ });
            Ok(())
        }
    }
};
