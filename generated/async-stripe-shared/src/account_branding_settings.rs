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
    icon: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    logo: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    primary_color: Option<Option<String>>,
    secondary_color: Option<Option<String>>,
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
                builder: AccountBrandingSettingsBuilder {
                    icon: Deserialize::default(),
                    logo: Deserialize::default(),
                    primary_color: Deserialize::default(),
                    secondary_color: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "icon" => Deserialize::begin(&mut self.builder.icon),
                "logo" => Deserialize::begin(&mut self.builder.logo),
                "primary_color" => Deserialize::begin(&mut self.builder.primary_color),
                "secondary_color" => Deserialize::begin(&mut self.builder.secondary_color),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(icon), Some(logo), Some(primary_color), Some(secondary_color)) = (
                self.builder.icon.take(),
                self.builder.logo.take(),
                self.builder.primary_color.take(),
                self.builder.secondary_color.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(AccountBrandingSettings { icon, logo, primary_color, secondary_color });
            Ok(())
        }
    }
};
