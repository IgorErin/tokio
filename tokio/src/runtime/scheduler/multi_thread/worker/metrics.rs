use super::{GroupIndex, Shared};

impl Shared {
    pub(crate) fn injection_queue_depth(&self, group: usize) -> usize {
        let group_index = GroupIndex::new(group);
        self.inject(group_index).len()
    }
}

cfg_unstable_metrics! {
    impl Shared {
        pub(crate) fn worker_local_queue_depth(&self, worker: usize) -> usize {
            self.remotes[worker].steal.len()
        }
    }
}
