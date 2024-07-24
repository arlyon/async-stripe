/// The resource representing a Stripe Polymorphic
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum ExternalAccount {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
}

#[derive(Default)]
pub struct ExternalAccountBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<ExternalAccount>,
        builder: ExternalAccountBuilder,
    }

    impl Deserialize for ExternalAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<ExternalAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for ExternalAccountBuilder {
        type Out = ExternalAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            ExternalAccount::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for ExternalAccount {
        type Builder = ExternalAccountBuilder;
    }
    impl ExternalAccount {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for ExternalAccount {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

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
