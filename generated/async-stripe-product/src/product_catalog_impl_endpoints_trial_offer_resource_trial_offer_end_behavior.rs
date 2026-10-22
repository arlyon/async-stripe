#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
    pub transition: Option<stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTransition>,
    /// The type of behavior when the trial offer ends.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder {
    transition:
        Option<Option<stripe_product::ProductCatalogImplEndpointsTrialOfferResourceTransition>>,
    type_: Option<ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType>,
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

    impl Deserialize for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior>,
        builder: ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder,
    }

    impl Visitor for Place<ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder {
        type Out = ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "transition" => Deserialize::begin(&mut self.transition),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { transition: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(transition), Some(type_)) = (self.transition.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { transition, type_ })
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

    impl ObjectDeser for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
        type Builder = ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder;
    }

    impl FromValueOpt for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehavior {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "transition" => b.transition = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of behavior when the trial offer ends.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    Transition,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    pub fn as_str(&self) -> &str {
        use ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::*;
        match self {
            Transition => "transition",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::*;
        match s {
            "transition" => Ok(Transition),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ProductCatalogImplEndpointsTrialOfferResourceTrialOfferEndBehaviorType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
