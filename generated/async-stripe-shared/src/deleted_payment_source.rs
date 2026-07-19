/// The resource representing a Stripe Polymorphic
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum DeletedPaymentSource {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    DeletedBankAccount(stripe_shared::DeletedBankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    DeletedCard(stripe_shared::DeletedCard),
}

const _: () = {
    use stripe_miniserde::de::Visitor;
    use stripe_miniserde::{Deserialize, Error, Result, make_place};
    use stripe_miniserde::json::peek_object_tag;

    use super::*;

    make_place!(Place);

    impl Deserialize for DeletedPaymentSource {
        const WANTS_RAW: bool = true;

        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DeletedPaymentSource> {
        fn wants_raw(&self) -> bool {
            true
        }

        fn raw(&mut self, bytes: &str) -> Result<()> {
            let tag = peek_object_tag(bytes).ok_or(Error)?;
            self.out = Some(match tag.as_str() {
                "bank_account" => {
                    DeletedPaymentSource::DeletedBankAccount(stripe_miniserde::json::from_str(bytes)?)
                }
                "card" => DeletedPaymentSource::DeletedCard(stripe_miniserde::json::from_str(bytes)?),

                _ => return Err(Error),
            });
            Ok(())
        }
    }
};

#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeletedPaymentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeletedPaymentSource").finish_non_exhaustive()
    }
}
impl stripe_types::Object for DeletedPaymentSource {
    type Id = smol_str::SmolStr;
    fn id(&self) -> &Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.inner(),
            Self::DeletedCard(v) => v.id.inner(),
        }
    }

    fn into_id(self) -> Self::Id {
        match self {
            Self::DeletedBankAccount(v) => v.id.into_inner(),
            Self::DeletedCard(v) => v.id.into_inner(),
        }
    }
}
