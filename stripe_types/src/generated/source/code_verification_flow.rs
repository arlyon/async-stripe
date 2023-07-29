#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeVerificationFlow {
    /// The number of attempts remaining to authenticate the source object with a verification code.
    pub attempts_remaining: i64,
    /// The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater than 0), `succeeded` (successful verification) or `failed` (failed verification, cannot be verified anymore as `attempts_remaining` should be 0).
    pub status: String,
}
