/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum DeletedExternalAccount {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "bank_account"))]
    DeletedBankAccount(stripe_shared::DeletedBankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    DeletedCard(stripe_shared::DeletedCard),
}

#[derive(Default)]
pub struct DeletedExternalAccountBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<DeletedExternalAccount>,
        builder: DeletedExternalAccountBuilder,
    }

    impl Deserialize for DeletedExternalAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DeletedExternalAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
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

    impl MapBuilder for DeletedExternalAccountBuilder {
        type Out = DeletedExternalAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            DeletedExternalAccount::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for DeletedExternalAccount {
        type Builder = DeletedExternalAccountBuilder;
    }
    impl DeletedExternalAccount {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => {
                    Self::DeletedBankAccount(FromValueOpt::from_value(Value::Object(o))?)
                }
                "card" => Self::DeletedCard(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for DeletedExternalAccount {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

impl stripe_types::Object for DeletedExternalAccount {
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
