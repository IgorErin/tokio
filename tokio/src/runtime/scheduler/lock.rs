/// A lock (mutex) yielding generic data.
use super::multi_thread::GroupIndex;

pub(crate) trait LockShard<T> {
    type Handle: AsMut<T>;

    fn lock(self, group: GroupIndex) -> Self::Handle;
}
