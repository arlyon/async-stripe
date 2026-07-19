/// The resource representing a Stripe Polymorphic
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum ExternalAccount {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
}

const _: () = {
    use stripe_miniserde::de::Visitor;
    use stripe_miniserde::{Deserialize, Error, Result, make_place};
    use stripe_miniserde::json::peek_object_tag;

    use super::*;

    make_place!(Place);

    impl Deserialize for ExternalAccount {
        const WANTS_RAW: bool = true;

        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<ExternalAccount> {
        fn wants_raw(&self) -> bool {
            true
        }

        fn raw(&mut self, bytes: &str) -> Result<()> {
            let tag = peek_object_tag(bytes).ok_or(Error)?;
            self.out = Some(match tag.as_str() {
                "bank_account" => ExternalAccount::BankAccount(stripe_miniserde::json::from_str(bytes)?),
                "card" => ExternalAccount::Card(stripe_miniserde::json::from_str(bytes)?),

                _ => return Err(Error),
            });
            Ok(())
        }
    }
};

#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ExternalAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExternalAccount").finish_non_exhaustive()
    }
}
impl stripe_types::Object for ExternalAccount {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::BankAccount(v) => v.id.inner(),
            Self::Card(v) => v.id.inner(),
        }
    }

    fn into_id(self) -> Self::Id {
        match self {
            Self::BankAccount(v) => v.id.into_inner(),
            Self::Card(v) => v.id.into_inner(),
        }
    }
}
