#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionBrandingSettings {
    /// A hex color value starting with `#` representing the background color for the Checkout Session.
    pub background_color: String,
    /// The border style for the Checkout Session. Must be one of `rounded`, `rectangular`, or `pill`.
    pub border_style: PaymentPagesCheckoutSessionBrandingSettingsBorderStyle,
    /// A hex color value starting with `#` representing the button color for the Checkout Session.
    pub button_color: String,
    /// The display name shown on the Checkout Session.
    pub display_name: String,
    /// The font family for the Checkout Session.
    /// Must be one of the [supported font families](https://docs.stripe.com/payments/checkout/customization/appearance?payment-ui=stripe-hosted#font-compatibility).
    pub font_family: String,
    /// The icon for the Checkout Session. You cannot set both `logo` and `icon`.
    pub icon: Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettingsIcon>,
    /// The logo for the Checkout Session. You cannot set both `logo` and `icon`.
    pub logo: Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettingsLogo>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionBrandingSettingsBuilder {
    background_color: Option<String>,
    border_style: Option<PaymentPagesCheckoutSessionBrandingSettingsBorderStyle>,
    button_color: Option<String>,
    display_name: Option<String>,
    font_family: Option<String>,
    icon: Option<Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettingsIcon>>,
    logo: Option<Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettingsLogo>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionBrandingSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionBrandingSettings>,
        builder: PaymentPagesCheckoutSessionBrandingSettingsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionBrandingSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionBrandingSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionBrandingSettingsBuilder {
        type Out = PaymentPagesCheckoutSessionBrandingSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "background_color" => Deserialize::begin(&mut self.background_color),
                "border_style" => Deserialize::begin(&mut self.border_style),
                "button_color" => Deserialize::begin(&mut self.button_color),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "font_family" => Deserialize::begin(&mut self.font_family),
                "icon" => Deserialize::begin(&mut self.icon),
                "logo" => Deserialize::begin(&mut self.logo),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                background_color: Deserialize::default(),
                border_style: Deserialize::default(),
                button_color: Deserialize::default(),
                display_name: Deserialize::default(),
                font_family: Deserialize::default(),
                icon: Deserialize::default(),
                logo: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(background_color),
                Some(border_style),
                Some(button_color),
                Some(display_name),
                Some(font_family),
                Some(icon),
                Some(logo),
            ) = (
                self.background_color.take(),
                self.border_style,
                self.button_color.take(),
                self.display_name.take(),
                self.font_family.take(),
                self.icon.take(),
                self.logo.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                background_color,
                border_style,
                button_color,
                display_name,
                font_family,
                icon,
                logo,
            })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionBrandingSettings {
        type Builder = PaymentPagesCheckoutSessionBrandingSettingsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionBrandingSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionBrandingSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "background_color" => b.background_color = FromValueOpt::from_value(v),
                    "border_style" => b.border_style = FromValueOpt::from_value(v),
                    "button_color" => b.button_color = FromValueOpt::from_value(v),
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "font_family" => b.font_family = FromValueOpt::from_value(v),
                    "icon" => b.icon = FromValueOpt::from_value(v),
                    "logo" => b.logo = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The border style for the Checkout Session. Must be one of `rounded`, `rectangular`, or `pill`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    Pill,
    Rectangular,
    Rounded,
}
impl PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::*;
        match self {
            Pill => "pill",
            Rectangular => "rectangular",
            Rounded => "rounded",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::*;
        match s {
            "pill" => Ok(Pill),
            "rectangular" => Ok(Rectangular),
            "rounded" => Ok(Rounded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionBrandingSettingsBorderStyle>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionBrandingSettingsBorderStyle);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle",
            )
        })
    }
}
