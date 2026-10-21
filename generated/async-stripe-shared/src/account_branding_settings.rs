#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountBrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    /// Must be square and at least 128px x 128px.
    pub icon: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    /// Must be at least 128px x 128px.
    pub logo: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// A CSS hex color value representing the primary branding color for this account
    pub primary_color: Option<String>,
    /// A CSS hex color value representing the secondary branding color for this account
    pub secondary_color: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountBrandingSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountBrandingSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountBrandingSettingsBuilder {
    icon: Option<stripe_types::Expandable<stripe_shared::File>>,
    logo: Option<stripe_types::Expandable<stripe_shared::File>>,
    primary_color: Option<Option<String>>,
    secondary_color: Option<Option<String>>,
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

    impl Deserialize for AccountBrandingSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBrandingSettings>,
        builder: AccountBrandingSettingsBuilder,
    }

    impl Visitor for Place<AccountBrandingSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountBrandingSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountBrandingSettingsBuilder {
        type Out = AccountBrandingSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "icon" => stripe_types::miniserde_helpers::begin_nullable(&mut self.icon),
                "logo" => stripe_types::miniserde_helpers::begin_nullable(&mut self.logo),
                "primary_color" => Deserialize::begin(&mut self.primary_color),
                "secondary_color" => Deserialize::begin(&mut self.secondary_color),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { icon: None, logo: None, primary_color: Some(None), secondary_color: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    icon,
                    logo,
                    primary_color: Some(primary_color),
                    secondary_color: Some(secondary_color),
                } => Some(Self::Out { icon, logo, primary_color, secondary_color }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for AccountBrandingSettings {
        type Builder = AccountBrandingSettingsBuilder;
    }

    impl FromValueOpt for AccountBrandingSettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
