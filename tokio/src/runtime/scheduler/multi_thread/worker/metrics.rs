use super::Shared;

impl Shared {
    pub(crate) fn injection_queue_depth(&self, group: usize) -> usize {
        self.groups[group].inject.len()
    }
}

cfg_unstable_metrics! {
    impl Shared {
        pub(crate) fn worker_local_queue_depth(&self, worker: usize) -> usize {
            let group_size = self.group_size;

            let group = worker / group_size;
            let worker = worker % group_size;

            self.groups[group].remotes[worker].steal.len()
        }
    }
}
