use chrono;
use std::fmt::{Display, Formatter};

pub struct Task {
    id: i64,
    pub name: String,
    pub done: bool,
    pub deadline: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl Task {
    pub fn new(id: i64, name: String, done: bool, deadline: Option<chrono::DateTime<chrono::FixedOffset>>) -> Self {
        Task {
            id,
            name,
            done,
            deadline,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Task(id={:?}, name={:?}, done={:?}, deadline={:?})",
            self.id, self.name, self.done, self.deadline
        )
    }
}
