#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PayoutsTraceId {
    /// Possible values are `pending`, `supported`, and `unsupported`.
    /// When `payout.status` is `pending` or `in_transit`, this will be `pending`.
    /// When the payout transitions to `paid`, `failed`, or `canceled`, this status will become `supported` or `unsupported` shortly after in most cases.
    /// In some cases, this may appear as `pending` for up to 10 days after `arrival_date` until transitioning to `supported` or `unsupported`.
    pub status: String,
    /// The trace ID value if `trace_id.status` is `supported`, otherwise `nil`.
    pub value: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PayoutsTraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PayoutsTraceId").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PayoutsTraceIdBuilder {
    status: Option<String>,
    value: Option<Option<String>>,
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

    impl Deserialize for PayoutsTraceId {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PayoutsTraceId>,
        builder: PayoutsTraceIdBuilder,
    }

    impl Visitor for Place<PayoutsTraceId> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PayoutsTraceIdBuilder {
                    status: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.builder.status),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(status), Some(value)) =
                (self.builder.status.take(), self.builder.value.take())
            else {
                return Ok(());
            };
            *self.out = Some(PayoutsTraceId { status, value });
            Ok(())
        }
    }
};
