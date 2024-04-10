#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxDeductedAtSourceId,
    /// The end of the invoicing period.
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: stripe_types::Timestamp,
    /// The start of the invoicing period.
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: stripe_types::Timestamp,
    /// The TAN that was supplied to Stripe when TDS was assessed
    pub tax_deduction_account_number: String,
}
#[doc(hidden)]
pub struct TaxDeductedAtSourceBuilder {
    id: Option<stripe_shared::TaxDeductedAtSourceId>,
    period_end: Option<stripe_types::Timestamp>,
    period_start: Option<stripe_types::Timestamp>,
    tax_deduction_account_number: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxDeductedAtSource {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxDeductedAtSource>,
        builder: TaxDeductedAtSourceBuilder,
    }

    impl Visitor for Place<TaxDeductedAtSource> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxDeductedAtSourceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxDeductedAtSourceBuilder {
        type Out = TaxDeductedAtSource;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "period_end" => Deserialize::begin(&mut self.period_end),
                "period_start" => Deserialize::begin(&mut self.period_start),
                "tax_deduction_account_number" => {
                    Deserialize::begin(&mut self.tax_deduction_account_number)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                period_end: Deserialize::default(),
                period_start: Deserialize::default(),
                tax_deduction_account_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                id: self.id.take()?,
                period_end: self.period_end?,
                period_start: self.period_start?,
                tax_deduction_account_number: self.tax_deduction_account_number.take()?,
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

    impl ObjectDeser for TaxDeductedAtSource {
        type Builder = TaxDeductedAtSourceBuilder;
    }

    impl FromValueOpt for TaxDeductedAtSource {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxDeductedAtSourceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "period_end" => b.period_end = Some(FromValueOpt::from_value(v)?),
                    "period_start" => b.period_start = Some(FromValueOpt::from_value(v)?),
                    "tax_deduction_account_number" => {
                        b.tax_deduction_account_number = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxDeductedAtSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxDeductedAtSource", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("period_end", &self.period_end)?;
        s.serialize_field("period_start", &self.period_start)?;
        s.serialize_field("tax_deduction_account_number", &self.tax_deduction_account_number)?;

        s.serialize_field("object", "tax_deducted_at_source")?;
        s.end()
    }
}
impl stripe_types::Object for TaxDeductedAtSource {
    type Id = stripe_shared::TaxDeductedAtSourceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId);
