use super::*;
use crate::inputs::key::Key;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

// #[test]
// fn should_find_action_by_key() {
//     let actions: Actions = vec![Action::Quit].into();
//     let result = actions.find(Key::Ctrl('c'));
//     assert_eq!(result, Some(&Action::Quit));
// }

// #[test]
// fn should_find_action_by_key_not_found() {
//     let actions: Actions = vec![Action::Quit].into();
//     let result = actions.find(Key::Alt('w'));
//     assert_eq!(result, None);
// }

// #[test]
// fn should_create_actions_from_vec() {
//     let _actions: Actions = vec![
//         Action::Quit,
//         Action::SwitchEditMode,
//         Action::SwitchNormalMode,
//     ]
//     .into();
// }

// #[test]
// #[should_panic]
// fn should_panic_when_create_actions_conflict_key() {
//     let _actions: Actions = vec![
//         Action::Quit,
//         Action::SwitchNormalMode,
//         Action::SwitchEditMode,
//         Action::SwitchEditMode,
//         Action::Quit,
//         Action::SwitchNormalMode,
//     ]
//     .into();
// }
