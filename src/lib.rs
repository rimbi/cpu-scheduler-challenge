use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.queued_at.cmp(&other.queued_at) {
            Ordering::Equal => self.execution_duration.cmp(&other.execution_duration),
            r @ _ => r,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.queued_at == other.queued_at && self.execution_duration == other.execution_duration
    }
}

pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    let mut tasks = tasks;
    let mut result = vec![];
    let mut time = 0;
    while !tasks.is_empty() {
        tasks.sort();
        let task = tasks.first().unwrap();
        result.push(task.id);
        time += task.queued_at + task.execution_duration;
        tasks.drain(0..1);
        for t in &mut tasks {
            if time <= t.queued_at {
                break;
            }
            t.queued_at = time;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_queue_order() {
        let tasks = vec![
            Task { id: 42, queued_at: 5, execution_duration: 3 },
            Task { id: 43, queued_at: 2, execution_duration: 3 },
            Task { id: 44, queued_at: 0, execution_duration: 2 },
        ];

        assert_eq!(execution_order(tasks), vec![44, 43, 42]);
    }

    #[test]
    fn two_items_queued_at_once() {
        // 0: #42 is queued
        // 0: #42 is started
        // 1: #43 is queued
        // 2: #44 is queued
        // 3: #42 is finished
        // 3: #44 is started (it is queued and has a lower execution_duration than #43)
        // 5: #44 is finished
        // 5: #43 is started
        // 8: #43 is finished

        let tasks = vec![
            Task { id: 42, queued_at: 0, execution_duration: 3 },
            Task { id: 43, queued_at: 1, execution_duration: 3 },
            Task { id: 44, queued_at: 2, execution_duration: 2 },
        ];

        assert_eq!(execution_order(tasks), vec![42, 44, 43]);
    }
}
