#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationNetworkData {
    /// Identifier assigned to the acquirer by the card network.
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
    /// The System Trace Audit Number (STAN) is a 6-digit identifier assigned by the acquirer.
    /// Prefer `network_data.transaction_id` if present, unless you have special requirements.
    pub system_trace_audit_number: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationNetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationNetworkData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationNetworkDataBuilder {
    acquiring_institution_id: Option<Option<String>>,
    system_trace_audit_number: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationNetworkData>,
        builder: IssuingAuthorizationNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationNetworkDataBuilder {
                    acquiring_institution_id: Deserialize::default(),
                    system_trace_audit_number: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acquiring_institution_id" => {
                    Deserialize::begin(&mut self.builder.acquiring_institution_id)
                }
                "system_trace_audit_number" => {
                    Deserialize::begin(&mut self.builder.system_trace_audit_number)
                }
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acquiring_institution_id),
                Some(system_trace_audit_number),
                Some(transaction_id),
            ) = (
                self.builder.acquiring_institution_id.take(),
                self.builder.system_trace_audit_number.take(),
                self.builder.transaction_id.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationNetworkData {
                acquiring_institution_id,
                system_trace_audit_number,
                transaction_id,
            });
            Ok(())
        }
    }
};
