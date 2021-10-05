#[derive(Debug)]
pub enum Status<P, E> {
    Launched(P),

    SetupFailed(E),
}
