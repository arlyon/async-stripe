#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxDeductedAtSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxDeductedAtSource").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxDeductedAtSourceBuilder {
    id: Option<stripe_shared::TaxDeductedAtSourceId>,
    period_end: Option<stripe_types::Timestamp>,
    period_start: Option<stripe_types::Timestamp>,
    tax_deduction_account_number: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TaxDeductedAtSourceBuilder {
                    id: Deserialize::default(),
                    period_end: Deserialize::default(),
                    period_start: Deserialize::default(),
                    tax_deduction_account_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "period_end" => Deserialize::begin(&mut self.builder.period_end),
                "period_start" => Deserialize::begin(&mut self.builder.period_start),
                "tax_deduction_account_number" => {
                    Deserialize::begin(&mut self.builder.tax_deduction_account_number)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(id),
                Some(period_end),
                Some(period_start),
                Some(tax_deduction_account_number),
            ) = (
                self.builder.id.take(),
                self.builder.period_end,
                self.builder.period_start,
                self.builder.tax_deduction_account_number.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxDeductedAtSource {
                id,
                period_end,
                period_start,
                tax_deduction_account_number,
            });
            Ok(())
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

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxDeductedAtSourceId);
