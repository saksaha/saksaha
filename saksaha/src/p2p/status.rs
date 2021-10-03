/// M message
pub enum Status<M> {
    Launched,

    SetupFailed(M),
}
