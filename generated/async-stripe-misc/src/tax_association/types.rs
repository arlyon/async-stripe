/// A Tax Association exposes the Tax Transactions that Stripe attempted to create on your behalf based on the PaymentIntent input.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxAssociation {
    /// The [Tax Calculation](https://docs.stripe.com/api/tax/calculations/object) that was included in PaymentIntent.
    pub calculation: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxAssociationId,
    /// The [PaymentIntent](https://docs.stripe.com/api/payment_intents/object) that this Tax Association is tracking.
    pub payment_intent: String,
    /// Information about the tax transactions linked to this payment intent
    pub tax_transaction_attempts:
        Option<Vec<stripe_misc::TaxProductResourceTaxAssociationTransactionAttempts>>,
}
#[doc(hidden)]
pub struct TaxAssociationBuilder {
    calculation: Option<String>,
    id: Option<stripe_misc::TaxAssociationId>,
    payment_intent: Option<String>,
    tax_transaction_attempts:
        Option<Option<Vec<stripe_misc::TaxProductResourceTaxAssociationTransactionAttempts>>>,
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

    impl Deserialize for TaxAssociation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxAssociation>,
        builder: TaxAssociationBuilder,
    }

    impl Visitor for Place<TaxAssociation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxAssociationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxAssociationBuilder {
        type Out = TaxAssociation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "calculation" => Deserialize::begin(&mut self.calculation),
                "id" => Deserialize::begin(&mut self.id),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "tax_transaction_attempts" => {
                    Deserialize::begin(&mut self.tax_transaction_attempts)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                calculation: Deserialize::default(),
                id: Deserialize::default(),
                payment_intent: Deserialize::default(),
                tax_transaction_attempts: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(calculation), Some(id), Some(payment_intent), Some(tax_transaction_attempts)) = (
                self.calculation.take(),
                self.id.take(),
                self.payment_intent.take(),
                self.tax_transaction_attempts.take(),
            ) else {
                return None;
            };
            Some(Self::Out { calculation, id, payment_intent, tax_transaction_attempts })
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

    impl ObjectDeser for TaxAssociation {
        type Builder = TaxAssociationBuilder;
    }

    impl FromValueOpt for TaxAssociation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxAssociationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "calculation" => b.calculation = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "tax_transaction_attempts" => {
                        b.tax_transaction_attempts = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxAssociation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxAssociation", 5)?;
        s.serialize_field("calculation", &self.calculation)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("tax_transaction_attempts", &self.tax_transaction_attempts)?;

        s.serialize_field("object", "tax.association")?;
        s.end()
    }
}
impl stripe_types::Object for TaxAssociation {
    type Id = stripe_misc::TaxAssociationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxAssociationId);
