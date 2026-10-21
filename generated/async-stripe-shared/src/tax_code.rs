/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
///
/// For more details see <<https://stripe.com/docs/api/tax_codes/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxCodeBuilder {
    description: Option<String>,
    id: Option<stripe_shared::TaxCodeId>,
    name: Option<String>,
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

    impl Deserialize for TaxCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCode>,
        builder: TaxCodeBuilder,
    }

    impl Visitor for Place<TaxCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxCodeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxCodeBuilder {
        type Out = TaxCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "id" => Deserialize::begin(&mut self.id),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { description: None, id: None, name: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { description: Some(description), id: Some(id), name: Some(name) } => {
                    Some(Self::Out { description, id, name })
                }
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

    impl ObjectDeser for TaxCode {
        type Builder = TaxCodeBuilder;
    }

    impl FromValueOpt for TaxCode {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxCode", 4)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("name", &self.name)?;

        s.serialize_field("object", "tax_code")?;
        s.end()
    }
}
impl stripe_types::Object for TaxCode {
    type Id = stripe_shared::TaxCodeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxCodeId);
