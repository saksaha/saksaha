use crate::inputs::key::Key;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

/// We define all available action
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KeyedAction {
    Quit,
    SwitchEditMode,
    SwitchNormalMode,
    ShowChList,
    ShowOpenCh,
    ShowChat,
    Down,
    Up,
    //
    UpdateBalance,
    Select,
    RestoreChat,
}

impl KeyedAction {
    /// All available actions
    pub fn iterator() -> Iter<'static, KeyedAction> {
        static ACTIONS: [KeyedAction; 11] = [
            KeyedAction::Quit,
            KeyedAction::SwitchEditMode,
            KeyedAction::SwitchNormalMode,
            KeyedAction::ShowChList,
            KeyedAction::ShowOpenCh,
            KeyedAction::ShowChat,
            KeyedAction::Down,
            KeyedAction::Up,
            KeyedAction::UpdateBalance,
            KeyedAction::Select,
            KeyedAction::RestoreChat,
        ];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            KeyedAction::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            KeyedAction::SwitchEditMode => &[Key::Char('i')],
            KeyedAction::SwitchNormalMode => &[Key::Esc],
            KeyedAction::ShowChList => &[Key::Char('1')],
            KeyedAction::ShowOpenCh => &[Key::Char('2')],
            KeyedAction::ShowChat => &[Key::Char('3')],
            KeyedAction::Down => &[Key::Down],
            KeyedAction::Up => &[Key::Up],
            KeyedAction::UpdateBalance => &[Key::Char('$')],
            KeyedAction::Select => &[Key::Enter],
            KeyedAction::RestoreChat => &[Key::Char('R')],
        }
    }
}

impl Display for KeyedAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            KeyedAction::Quit => "Quit",
            KeyedAction::SwitchEditMode => "Switch to Edit Mode",
            KeyedAction::SwitchNormalMode => "Switch to Normal Mode",
            KeyedAction::ShowChList => "Show channel list",
            KeyedAction::ShowOpenCh => "Show open channel",
            KeyedAction::ShowChat => "Show chatting",
            KeyedAction::Down => "Down",
            KeyedAction::Up => "Up",
            KeyedAction::UpdateBalance => "Show my balance in wallet",
            KeyedAction::Select => "Select",
            KeyedAction::RestoreChat => "Restore the chats",
        };
        write!(f, "{}", str)
    }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(pub Vec<KeyedAction>);

impl Actions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&KeyedAction> {
        KeyedAction::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[KeyedAction] {
        self.0.as_slice()
    }
}

impl From<Vec<KeyedAction>> for Actions {
    /// Build contextual action
    ///
    /// # Panics
    ///
    /// If two actions have same key
    fn from(actions: Vec<KeyedAction>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<KeyedAction>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(KeyedAction::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_action_by_key() {
        let actions: Actions = vec![KeyedAction::Quit].into();
        let result = actions.find(Key::Ctrl('c'));
        assert_eq!(result, Some(&KeyedAction::Quit));
    }

    #[test]
    fn should_find_action_by_key_not_found() {
        let actions: Actions = vec![KeyedAction::Quit].into();
        let result = actions.find(Key::Alt('w'));
        assert_eq!(result, None);
    }

    #[test]
    fn should_create_actions_from_vec() {
        let _actions: Actions = vec![
            KeyedAction::Quit,
            KeyedAction::SwitchEditMode,
            KeyedAction::SwitchNormalMode,
        ]
        .into();
    }

    #[test]
    #[should_panic]
    fn should_panic_when_create_actions_conflict_key() {
        let _actions: Actions = vec![
            KeyedAction::Quit,
            KeyedAction::SwitchNormalMode,
            KeyedAction::SwitchEditMode,
            KeyedAction::SwitchEditMode,
            KeyedAction::Quit,
            KeyedAction::SwitchNormalMode,
        ]
        .into();
    }
}
