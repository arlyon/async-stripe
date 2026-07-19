#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceRedirectFlow {
    /// The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), `declined` (the authentication failed or the transaction was declined), or `processing_error` (the redirect failed due to a technical error).
    /// Present only if the redirect status is `failed`.
    pub failure_reason: Option<String>,
    /// The URL you provide to redirect the customer to after they authenticated their payment.
    pub return_url: String,
    /// The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `succeeded` (successful authentication, cannot be reused) or `not_required` (redirect should not be used) or `failed` (failed authentication, cannot be reused).
    pub status: String,
    /// The URL provided to you to redirect a customer to as part of a `redirect` authentication flow.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceRedirectFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceRedirectFlow").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceRedirectFlowBuilder {
    failure_reason: Option<Option<String>>,
    return_url: Option<String>,
    status: Option<String>,
    url: Option<String>,
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

    impl Deserialize for SourceRedirectFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceRedirectFlow>,
        builder: SourceRedirectFlowBuilder,
    }

    impl Visitor for Place<SourceRedirectFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceRedirectFlowBuilder {
                    failure_reason: Deserialize::default(),
                    return_url: Deserialize::default(),
                    status: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "failure_reason" => Deserialize::begin(&mut self.builder.failure_reason),
                "return_url" => Deserialize::begin(&mut self.builder.return_url),
                "status" => Deserialize::begin(&mut self.builder.status),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(failure_reason), Some(return_url), Some(status), Some(url)) = (
                self.builder.failure_reason.take(),
                self.builder.return_url.take(),
                self.builder.status.take(),
                self.builder.url.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceRedirectFlow { failure_reason, return_url, status, url });
            Ok(())
        }
    }
};
