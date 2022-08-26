// pub mod handler;

// #[derive(Debug, Clone)]
// pub enum IoEvent {
//     Initialize,
//     GetChList(Vec<u8>),
//     GetMessages(Vec<u8>),
// }

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}
