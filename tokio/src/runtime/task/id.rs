use crate::runtime::context;

use std::{fmt, num::NonZeroU64};

use crate::loom::sync::atomic::Ordering::Relaxed;
use crate::loom::sync::atomic::StaticAtomicU64;

/// TODO(i.Erin)
#[cfg_attr(docsrs, doc(cfg(all(feature = "rt"))))]
#[derive(Debug)]
pub struct IdProvider {
    global_id: NonZeroU64,
    local_counter: StaticAtomicU64,
}

fn new_nonzero_u64(atomic: &StaticAtomicU64) -> NonZeroU64 {
    loop {
        let id = atomic.fetch_add(1, Relaxed);
        if let Some(id) = NonZeroU64::new(id) {
            return id;
        }
    }
}

impl IdProvider {
    fn counter() -> &'static StaticAtomicU64 {
        static NEXT_ID: StaticAtomicU64 = StaticAtomicU64::new(1);
        &NEXT_ID
    }

    pub(crate) fn new() -> Self {
        Self {
            global_id: new_nonzero_u64(Self::counter()),
            local_counter: StaticAtomicU64::new(1),
        }
    }

    pub(crate) fn next_id(&self) -> Id {
        Id {
            global_id: self.global_id,
            local_id: new_nonzero_u64(&self.local_counter),
        }
    }

    pub(crate) fn new_id() -> Id {
        Id {
            global_id: new_nonzero_u64(Self::counter()),
            local_id: unsafe { NonZeroU64::new_unchecked(1) },
        }
    }
}

/// An opaque ID that uniquely identifies a task relative to all other currently
/// running tasks.
///
/// # Notes
///
/// - Task IDs are unique relative to other *currently running* tasks. When a
///   task completes, the same ID may be used for another task.
/// - Task IDs are *not* sequential, and do not indicate the order in which
///   tasks are spawned, what runtime a task is spawned on, or any other data.
/// - The task ID of the currently running task can be obtained from inside the
///   task via the [`task::try_id()`](crate::task::try_id()) and
///   [`task::id()`](crate::task::id()) functions and from outside the task via
///   the [`JoinHandle::id()`](crate::task::JoinHandle::id()) function.
#[cfg_attr(docsrs, doc(cfg(all(feature = "rt"))))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Id {
    global_id: NonZeroU64,
    pub(crate) local_id: NonZeroU64,
}

/// Returns the [`Id`] of the currently running task.
///
/// # Panics
///
/// This function panics if called from outside a task. Please note that calls
/// to `block_on` do not have task IDs, so the method will panic if called from
/// within a call to `block_on`. For a version of this function that doesn't
/// panic, see [`task::try_id()`](crate::runtime::task::try_id()).
///
/// [task ID]: crate::task::Id
#[track_caller]
pub fn id() -> Id {
    context::current_task_id().expect("Can't get a task id when not inside a task")
}

/// Returns the [`Id`] of the currently running task, or `None` if called outside
/// of a task.
///
/// This function is similar to  [`task::id()`](crate::runtime::task::id()), except
/// that it returns `None` rather than panicking if called outside of a task
/// context.
///
/// [task ID]: crate::task::Id
#[track_caller]
pub fn try_id() -> Option<Id> {
    context::current_task_id()
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.global_id, self.local_id)
    }
}

impl Id {
    // TODO(i.Erin)
    pub(crate) fn next() -> Id {
        IdProvider::new_id()
    }

    // TODO(i.Erin) replace with as_u128
    pub(crate) fn as_u64(&self) -> u64 {
        self.global_id.get()
    }
}
