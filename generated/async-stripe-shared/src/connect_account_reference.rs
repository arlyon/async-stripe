#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectAccountReference {
    /// The connected account being referenced when `type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Type of the account referenced.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateInvoiceIssuerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectAccountReference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectAccountReference").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectAccountReferenceBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    type_: Option<stripe_shared::CreateInvoiceIssuerType>,
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

    impl Deserialize for ConnectAccountReference {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectAccountReference>,
        builder: ConnectAccountReferenceBuilder,
    }

    impl Visitor for Place<ConnectAccountReference> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectAccountReferenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectAccountReferenceBuilder {
        type Out = ConnectAccountReference;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account), Some(type_)) = (self.account.take(), self.type_.take()) else {
                return None;
            };
            Some(Self::Out { account, type_ })
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

    impl ObjectDeser for ConnectAccountReference {
        type Builder = ConnectAccountReferenceBuilder;
    }

    impl FromValueOpt for ConnectAccountReference {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectAccountReferenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the account referenced.
pub type ConnectAccountReferenceType = stripe_shared::CreateInvoiceIssuerType;
