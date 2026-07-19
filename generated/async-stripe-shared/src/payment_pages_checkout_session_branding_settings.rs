#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionBrandingSettings").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentPagesCheckoutSessionBrandingSettingsBuilder {
                    background_color: Deserialize::default(),
                    border_style: Deserialize::default(),
                    button_color: Deserialize::default(),
                    display_name: Deserialize::default(),
                    font_family: Deserialize::default(),
                    icon: Deserialize::default(),
                    logo: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "background_color" => Deserialize::begin(&mut self.builder.background_color),
                "border_style" => Deserialize::begin(&mut self.builder.border_style),
                "button_color" => Deserialize::begin(&mut self.builder.button_color),
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "font_family" => Deserialize::begin(&mut self.builder.font_family),
                "icon" => Deserialize::begin(&mut self.builder.icon),
                "logo" => Deserialize::begin(&mut self.builder.logo),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(background_color),
                Some(border_style),
                Some(button_color),
                Some(display_name),
                Some(font_family),
                Some(icon),
                Some(logo),
            ) = (
                self.builder.background_color.take(),
                self.builder.border_style.take(),
                self.builder.button_color.take(),
                self.builder.display_name.take(),
                self.builder.font_family.take(),
                self.builder.icon.take(),
                self.builder.logo.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionBrandingSettings {
                background_color,
                border_style,
                button_color,
                display_name,
                font_family,
                icon,
                logo,
            });
            Ok(())
        }
    }
};
/// The border style for the Checkout Session. Must be one of `rounded`, `rectangular`, or `pill`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    Pill,
    Rectangular,
    Rounded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::*;
        match self {
            Pill => "pill",
            Rectangular => "rectangular",
            Rounded => "rounded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::*;
        match s {
            "pill" => Ok(Pill),
            "rectangular" => Ok(Rectangular),
            "rounded" => Ok(Rounded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionBrandingSettingsBorderStyle"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentPagesCheckoutSessionBrandingSettingsBorderStyle))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionBrandingSettingsBorderStyle>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionBrandingSettingsBorderStyle::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionBrandingSettingsBorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
