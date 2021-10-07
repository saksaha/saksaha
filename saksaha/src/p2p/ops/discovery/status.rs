pub enum Status<E> {
    Launched,

    SetupFailed(E),
}
