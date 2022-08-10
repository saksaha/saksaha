use crate::inputs::key::Key;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Sleep,
    SwitchEditMode,
    SwitchNormalMode,
    ShowChList,
    ShowOpenCh,
    ShowChat,
    ShowMyBalance,
    Down,
    Up,
    Right,
}

impl Action {
    /// All available actions
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 11] = [
            Action::Quit,
            Action::Sleep,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
            Action::ShowChList,
            Action::ShowOpenCh,
            Action::ShowChat,
            Action::ShowMyBalance,
            Action::Down,
            Action::Up,
            Action::Right,
        ];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            Action::Sleep => &[Key::Char('s')],
            Action::SwitchEditMode => &[Key::Char('i')],
            Action::SwitchNormalMode => &[Key::Esc],
            Action::ShowChList => &[Key::Char('1')],
            Action::ShowOpenCh => &[Key::Char('2')],
            Action::ShowChat => &[Key::Char('3')],
            Action::ShowMyBalance => &[Key::Char('$')],
            Action::Down => &[Key::Down],
            Action::Up => &[Key::Up],
            Action::Right => &[Key::Right],
        }
    }
}

/// Could display a user friendly short description of action
impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Sleep => "Sleep",
            Action::SwitchEditMode => "Switch to Edit Mode",
            Action::SwitchNormalMode => "Switch to Normal Mode",
            Action::ShowChList => "Show channel list",
            Action::ShowOpenCh => "Show open channel",
            Action::ShowChat => "Show chatting",
            Action::ShowMyBalance => "Show my balance in wallet",
            Action::Down => "Down",
            Action::Up => "Up",
            Action::Right => "move to chat room",
        };
        write!(f, "{}", str)
    }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    /// Build contextual action
    ///
    /// # Panics
    ///
    /// If two actions have same key
    fn from(actions: Vec<Action>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
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
                    .map(Action::to_string)
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
        let actions: Actions = vec![Action::Quit, Action::Sleep].into();
        let result = actions.find(Key::Ctrl('c'));
        assert_eq!(result, Some(&Action::Quit));
    }

    #[test]
    fn should_find_action_by_key_not_found() {
        let actions: Actions = vec![Action::Quit, Action::Sleep].into();
        let result = actions.find(Key::Alt('w'));
        assert_eq!(result, None);
    }

    #[test]
    fn should_create_actions_from_vec() {
        let _actions: Actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
        ]
        .into();
    }

    #[test]
    #[should_panic]
    fn should_panic_when_create_actions_conflict_key() {
        let _actions: Actions = vec![
            Action::Quit,
            Action::SwitchNormalMode,
            Action::Sleep,
            Action::SwitchEditMode,
            Action::SwitchEditMode,
            Action::Quit,
            Action::SwitchNormalMode,
        ]
        .into();
    }
}
