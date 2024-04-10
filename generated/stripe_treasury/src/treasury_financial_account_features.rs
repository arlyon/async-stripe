/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountFeatures {
    pub card_issuing: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub deposit_insurance: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub financial_addresses:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    pub inbound_transfers:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    pub intra_stripe_flows:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    pub outbound_payments:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    pub outbound_transfers:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountFeaturesBuilder {
    card_issuing: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    deposit_insurance:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    financial_addresses: Option<
        Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    >,
    inbound_transfers:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>>,
    intra_stripe_flows:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
    outbound_payments:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>>,
    outbound_transfers:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountFeatures>,
        builder: TreasuryFinancialAccountFeaturesBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountFeaturesBuilder {
        type Out = TreasuryFinancialAccountFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_issuing" => Deserialize::begin(&mut self.card_issuing),
                "deposit_insurance" => Deserialize::begin(&mut self.deposit_insurance),
                "financial_addresses" => Deserialize::begin(&mut self.financial_addresses),
                "inbound_transfers" => Deserialize::begin(&mut self.inbound_transfers),
                "intra_stripe_flows" => Deserialize::begin(&mut self.intra_stripe_flows),
                "outbound_payments" => Deserialize::begin(&mut self.outbound_payments),
                "outbound_transfers" => Deserialize::begin(&mut self.outbound_transfers),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_issuing: Deserialize::default(),
                deposit_insurance: Deserialize::default(),
                financial_addresses: Deserialize::default(),
                inbound_transfers: Deserialize::default(),
                intra_stripe_flows: Deserialize::default(),
                outbound_payments: Deserialize::default(),
                outbound_transfers: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                card_issuing: self.card_issuing.take()?,
                deposit_insurance: self.deposit_insurance.take()?,
                financial_addresses: self.financial_addresses.take()?,
                inbound_transfers: self.inbound_transfers.take()?,
                intra_stripe_flows: self.intra_stripe_flows.take()?,
                outbound_payments: self.outbound_payments.take()?,
                outbound_transfers: self.outbound_transfers.take()?,
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

    impl ObjectDeser for TreasuryFinancialAccountFeatures {
        type Builder = TreasuryFinancialAccountFeaturesBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_issuing" => b.card_issuing = Some(FromValueOpt::from_value(v)?),
                    "deposit_insurance" => b.deposit_insurance = Some(FromValueOpt::from_value(v)?),
                    "financial_addresses" => {
                        b.financial_addresses = Some(FromValueOpt::from_value(v)?)
                    }
                    "inbound_transfers" => b.inbound_transfers = Some(FromValueOpt::from_value(v)?),
                    "intra_stripe_flows" => {
                        b.intra_stripe_flows = Some(FromValueOpt::from_value(v)?)
                    }
                    "outbound_payments" => b.outbound_payments = Some(FromValueOpt::from_value(v)?),
                    "outbound_transfers" => {
                        b.outbound_transfers = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountFeatures {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TreasuryFinancialAccountFeatures", 8)?;
        s.serialize_field("card_issuing", &self.card_issuing)?;
        s.serialize_field("deposit_insurance", &self.deposit_insurance)?;
        s.serialize_field("financial_addresses", &self.financial_addresses)?;
        s.serialize_field("inbound_transfers", &self.inbound_transfers)?;
        s.serialize_field("intra_stripe_flows", &self.intra_stripe_flows)?;
        s.serialize_field("outbound_payments", &self.outbound_payments)?;
        s.serialize_field("outbound_transfers", &self.outbound_transfers)?;

        s.serialize_field("object", "treasury.financial_account_features")?;
        s.end()
    }
}
