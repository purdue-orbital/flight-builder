pub mod on_enter;
pub mod on_exit;
pub mod on_transition;

use crate::states::States;
use crate::tasks::StoredTask;
use alloc::vec::Vec;

pub use on_enter::OnEnter;
pub use on_exit::OnExit;
pub use on_transition::OnTransition;

pub struct Transition<S: States> {
    pub(super) on_enter: Vec<(S, Vec<StoredTask>)>,
    pub(super) on_exit: Vec<(S, Vec<StoredTask>)>,
    pub(super) on_transition: Vec<(S, S, Vec<StoredTask>)>,
}
