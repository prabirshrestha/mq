pub enum JobResult {
    CompleteWithSuccess,
    CompleteWithCancelled(Option<String>),
}
