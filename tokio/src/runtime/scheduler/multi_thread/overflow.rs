use crate::runtime::task;

#[cfg(test)]
use std::cell::RefCell;

use super::GroupIndex;

pub(crate) trait OverflowShard<T: 'static> {
    fn push(&self, task: task::Notified<T>, group: GroupIndex);

    fn push_batch<I>(&self, iter: I, group: GroupIndex)
    where
        I: Iterator<Item = task::Notified<T>>;
}

#[cfg(test)]
impl<T: 'static> OverflowShard<T> for RefCell<Vec<Vec<task::Notified<T>>>> {
    fn push(&self, task: task::Notified<T>, group: GroupIndex) {
        self.borrow_mut()[group.to_usize()].push(task);
    }

    fn push_batch<I>(&self, iter: I, group: GroupIndex)
    where
        I: Iterator<Item = task::Notified<T>>,
    {
        self.borrow_mut()[group.to_usize()].extend(iter);
    }
}
