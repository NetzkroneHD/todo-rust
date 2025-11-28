use chrono;
use std::fmt::{Display, Formatter};

pub struct Task {
    pub id: i64,
    pub name: String,
    pub done: bool,
    pub deadline: Option<chrono::DateTime<chrono::FixedOffset>>,
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
