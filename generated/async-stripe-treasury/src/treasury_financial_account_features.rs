/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountFeatures").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountFeaturesBuilder {
    card_issuing: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    deposit_insurance: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    financial_addresses:
        Option<stripe_treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    inbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    intra_stripe_flows: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    outbound_payments: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    outbound_transfers: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
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
                "card_issuing" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_issuing)
                }
                "deposit_insurance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.deposit_insurance)
                }
                "financial_addresses" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.financial_addresses)
                }
                "inbound_transfers" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.inbound_transfers)
                }
                "intra_stripe_flows" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.intra_stripe_flows)
                }
                "outbound_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.outbound_payments)
                }
                "outbound_transfers" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.outbound_transfers)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_issuing: None,
                deposit_insurance: None,
                financial_addresses: None,
                inbound_transfers: None,
                intra_stripe_flows: None,
                outbound_payments: None,
                outbound_transfers: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                card_issuing,
                deposit_insurance,
                financial_addresses,
                inbound_transfers,
                intra_stripe_flows,
                outbound_payments,
                outbound_transfers,
            ) = (
                self.card_issuing.take(),
                self.deposit_insurance.take(),
                self.financial_addresses.take(),
                self.inbound_transfers.take(),
                self.intra_stripe_flows.take(),
                self.outbound_payments.take(),
                self.outbound_transfers.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                card_issuing,
                deposit_insurance,
                financial_addresses,
                inbound_transfers,
                intra_stripe_flows,
                outbound_payments,
                outbound_transfers,
            })
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

    impl ObjectDeser for TreasuryFinancialAccountFeatures {
        type Builder = TreasuryFinancialAccountFeaturesBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountFeatures {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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
