#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::PlatformTaxFeeId,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct PlatformTaxFeeBuilder {
    account: Option<String>,
    id: Option<stripe_shared::PlatformTaxFeeId>,
    source_transaction: Option<String>,
    type_: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PlatformTaxFee {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PlatformTaxFee>,
        builder: PlatformTaxFeeBuilder,
    }

    impl Visitor for Place<PlatformTaxFee> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PlatformTaxFeeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PlatformTaxFeeBuilder {
        type Out = PlatformTaxFee;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "id" => Deserialize::begin(&mut self.id),
                "source_transaction" => Deserialize::begin(&mut self.source_transaction),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                id: Deserialize::default(),
                source_transaction: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account: self.account.take()?,
                id: self.id.take()?,
                source_transaction: self.source_transaction.take()?,
                type_: self.type_.take()?,
            })
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

    impl ObjectDeser for PlatformTaxFee {
        type Builder = PlatformTaxFeeBuilder;
    }

    impl FromValueOpt for PlatformTaxFee {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PlatformTaxFeeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "source_transaction" => {
                        b.source_transaction = Some(FromValueOpt::from_value(v)?)
                    }
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PlatformTaxFee {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PlatformTaxFee", 5)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("source_transaction", &self.source_transaction)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "platform_tax_fee")?;
        s.end()
    }
}
impl stripe_types::Object for PlatformTaxFee {
    type Id = stripe_shared::PlatformTaxFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PlatformTaxFeeId);
