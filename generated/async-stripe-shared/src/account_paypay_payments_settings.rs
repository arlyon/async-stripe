#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPaypayPaymentsSettings {
    /// Additional files that are required to support the onboarding process of your business.
    pub additional_files: Option<Vec<String>>,
    /// The type of goods your business sells.
    /// Use `digital_content` if you sell digital content.
    /// Use `other` for all other types of goods or services.
    pub goods_type: Option<AccountPaypayPaymentsSettingsGoodsType>,
    pub site: Option<stripe_shared::AccountPaypaySiteSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypayPaymentsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPaypayPaymentsSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountPaypayPaymentsSettingsBuilder {
    additional_files: Option<Option<Vec<String>>>,
    goods_type: Option<Option<AccountPaypayPaymentsSettingsGoodsType>>,
    site: Option<Option<stripe_shared::AccountPaypaySiteSettings>>,
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

    impl Deserialize for AccountPaypayPaymentsSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaypayPaymentsSettings>,
        builder: AccountPaypayPaymentsSettingsBuilder,
    }

    impl Visitor for Place<AccountPaypayPaymentsSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPaypayPaymentsSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPaypayPaymentsSettingsBuilder {
        type Out = AccountPaypayPaymentsSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional_files" => Deserialize::begin(&mut self.additional_files),
                "goods_type" => Deserialize::begin(&mut self.goods_type),
                "site" => Deserialize::begin(&mut self.site),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { additional_files: Some(None), goods_type: Some(None), site: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(additional_files), Some(goods_type), Some(site)) =
                (self.additional_files.take(), self.goods_type.take(), self.site.take())
            else {
                return None;
            };
            Some(Self::Out { additional_files, goods_type, site })
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

    impl ObjectDeser for AccountPaypayPaymentsSettings {
        type Builder = AccountPaypayPaymentsSettingsBuilder;
    }

    impl FromValueOpt for AccountPaypayPaymentsSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountPaypayPaymentsSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional_files" => b.additional_files = FromValueOpt::from_value(v),
                    "goods_type" => b.goods_type = FromValueOpt::from_value(v),
                    "site" => b.site = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of goods your business sells.
/// Use `digital_content` if you sell digital content.
/// Use `other` for all other types of goods or services.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountPaypayPaymentsSettingsGoodsType {
    DigitalContent,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountPaypayPaymentsSettingsGoodsType {
    pub fn as_str(&self) -> &str {
        use AccountPaypayPaymentsSettingsGoodsType::*;
        match self {
            DigitalContent => "digital_content",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountPaypayPaymentsSettingsGoodsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountPaypayPaymentsSettingsGoodsType::*;
        match s {
            "digital_content" => Ok(DigitalContent),
            "other" => Ok(Other),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountPaypayPaymentsSettingsGoodsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountPaypayPaymentsSettingsGoodsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountPaypayPaymentsSettingsGoodsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypayPaymentsSettingsGoodsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountPaypayPaymentsSettingsGoodsType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountPaypayPaymentsSettingsGoodsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountPaypayPaymentsSettingsGoodsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountPaypayPaymentsSettingsGoodsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountPaypayPaymentsSettingsGoodsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountPaypayPaymentsSettingsGoodsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountPaypayPaymentsSettingsGoodsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
