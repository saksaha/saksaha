use crate::inputs::key::Key;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    Quit,
    SwitchEditMode,
    SwitchNormalMode,
    ShowChList,
    ShowOpenCh,
    ShowChat,
    DownCh,
    UpCh,

    DownChat,
    UpChat,
    PageUpChat,
    //
    UpdateBalance,
    UpdateBalanceSuccess(u64),
    Select,
    RestoreChat,
    //
    Initialize,
    GetChList(Vec<u8>),
    GetMessages(Vec<u8>),
}

impl Action {
    /// All available actions
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 14] = [
            Action::Quit,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
            Action::ShowChList,
            Action::ShowOpenCh,
            Action::ShowChat,
            Action::DownCh,
            Action::UpCh,
            Action::DownChat,
            Action::UpChat,
            Action::PageUpChat,
            Action::Select,
            Action::RestoreChat,
            Action::UpdateBalance,
        ];
        ACTIONS.iter()
    }

    // List of key associated to action
    // pub fn keys(&self) -> &[Key] {
    //     match self {
    //         Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
    //         Action::SwitchEditMode => &[Key::Char('i')],
    //         Action::SwitchNormalMode => &[Key::Esc],
    //         Action::ShowChList => &[Key::Char('1')],
    //         Action::ShowOpenCh => &[Key::Char('2')],
    //         Action::ShowChat => &[Key::Char('3')],
    //         Action::DownCh => &[Key::Down],
    //         Action::UpCh => &[Key::Up],
    //         Action::DownChat => &[Key::Down],
    //         Action::UpChat => &[Key::Up],
    //         Action::PageUpChat => &[Key::PageUp],
    //         Action::RestoreChat => &[Key::Char('R')],
    //         Action::UpdateBalance => &[Key::Char('$')],
    //         Action::UpdateBalanceSuccess(_) => &[],
    //         Action::Select => &[Key::Enter],
    //         Action::Initialize => &[],
    //         Action::GetChList(_) => &[],
    //         Action::GetMessages(_) => &[],
    //     }
    // }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::SwitchEditMode => "Switch to Edit Mode",
            Action::SwitchNormalMode => "Switch to Normal Mode",
            Action::ShowChList => "Show channel list",
            Action::ShowOpenCh => "Show open channel",
            Action::ShowChat => "Show chatting",
            Action::DownCh => "DownCh",
            Action::UpCh => "UpCh",
            Action::DownChat => "DownChat",
            Action::UpChat => "UpChat",
            Action::PageUpChat => "PageUpChat",
            Action::Select => "Select",
            Action::RestoreChat => "Restore the chats",
            Action::Initialize => "Initialize",
            Action::GetChList(_) => "Get channel list",
            Action::GetMessages(_) => "Get messages in a channel",
            Action::UpdateBalance => "Show my balance in wallet",
            Action::UpdateBalanceSuccess(_) => "Show my balance in wallet",
        };
        write!(f, "{}", str)
    }
}

// The application should have some contextual actions.
// #[derive(Default, Debug, Clone)]
// pub struct Actions(pub Vec<Action>);

// impl Actions {
//     /// Given a key, find the corresponding action
//     pub fn find(&self, key: Key) -> Option<&Action> {
//         Action::iterator()
//             .filter(|action| self.0.contains(action))
//             .find(|action| action.keys().contains(&key))
//     }

//     /// Get contextual actions.
//     /// (just for building a help view)
//     pub fn actions(&self) -> &[Action] {
//         self.0.as_slice()
//     }
// }

// impl From<Vec<Action>> for Actions {
//     /// Build contextual action
//     ///
//     /// # Panics
//     ///
//     /// If two actions have same key
//     fn from(actions: Vec<Action>) -> Self {
//         // Check key unicity
//         let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
//         for action in actions.iter() {
//             for key in action.keys().iter() {
//                 match map.get_mut(key) {
//                     Some(vec) => vec.push(action.clone()),
//                     None => {
//                         map.insert(*key, vec![action.clone()]);
//                     }
//                 }
//             }
//         }

//         let errors = map
//             .iter()
//             .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
//             .map(|(key, actions)| {
//                 let actions = actions
//                     .iter()
//                     .map(Action::to_string)
//                     .collect::<Vec<_>>()
//                     .join(", ");
//                 format!("Conflict key {} with actions {}", key, actions)
//             })
//             .collect::<Vec<_>>();

//         if !errors.is_empty() {
//             panic!("{}", errors.join("; "))
//         }

//         // Ok, we can create contextual actions
//         Self(actions)
//     }
// }
