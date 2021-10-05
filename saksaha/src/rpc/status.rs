pub enum Status<T, E> {
    Launched(T),

    SetupFailed(E)
}
