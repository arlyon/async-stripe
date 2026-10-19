#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    pub filters:
        Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFilters>,
    /// The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<stripe_shared::Permissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<stripe_shared::Prefetch>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
    filters: Option<
        Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFilters>,
    >,
    permissions: Option<Option<Vec<stripe_shared::Permissions>>>,
    prefetch: Option<Option<Vec<stripe_shared::Prefetch>>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>,
        builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
        type Out = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.filters),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "prefetch" => Deserialize::begin(&mut self.prefetch),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { filters: Some(None), permissions: Some(None), prefetch: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(filters), Some(permissions), Some(prefetch)) =
                (self.filters.take(), self.permissions.take(), self.prefetch.take())
            else {
                return None;
            };
            Some(Self::Out { filters, permissions, prefetch })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        type Builder = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "prefetch" => b.prefetch = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
pub type InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions =
    stripe_shared::Permissions;
/// Data features requested to be retrieved upon account creation.
pub type InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch =
    stripe_shared::Prefetch;
