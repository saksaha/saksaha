#[derive(Debug)]
pub enum Status<M> {
    Launched,

    SetupFailed(M),
}
