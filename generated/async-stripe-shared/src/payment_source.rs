/// The resource representing a Stripe Polymorphic
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum PaymentSource {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "account"))]
    Account(stripe_shared::Account),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

const _: () = {
    use stripe_miniserde::de::Visitor;
    use stripe_miniserde::{Deserialize, Error, Result, make_place};
    use stripe_miniserde::json::peek_object_tag;

    use super::*;

    make_place!(Place);

    impl Deserialize for PaymentSource {
        const WANTS_RAW: bool = true;

        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<PaymentSource> {
        fn wants_raw(&self) -> bool {
            true
        }

        fn raw(&mut self, bytes: &str) -> Result<()> {
            let tag = peek_object_tag(bytes).ok_or(Error)?;
            self.out = Some(match tag.as_str() {
                "account" => PaymentSource::Account(stripe_miniserde::json::from_str(bytes)?),
                "bank_account" => PaymentSource::BankAccount(stripe_miniserde::json::from_str(bytes)?),
                "card" => PaymentSource::Card(stripe_miniserde::json::from_str(bytes)?),
                "source" => PaymentSource::Source(stripe_miniserde::json::from_str(bytes)?),

                _ => return Err(Error),
            });
            Ok(())
        }
    }
};

#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentSource").finish_non_exhaustive()
    }
}
impl stripe_types::Object for PaymentSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::Account(v) => v.id.inner(),
            Self::BankAccount(v) => v.id.inner(),
            Self::Card(v) => v.id.inner(),
            Self::Source(v) => v.id.inner(),
        }
    }

    fn into_id(self) -> Self::Id {
        match self {
            Self::Account(v) => v.id.into_inner(),
            Self::BankAccount(v) => v.id.into_inner(),
            Self::Card(v) => v.id.into_inner(),
            Self::Source(v) => v.id.into_inner(),
        }
    }
}
