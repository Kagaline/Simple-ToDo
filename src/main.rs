#[derive(Debug, PartialEq)]
enum Status {
    NotYet,
    Doing,
    Done,
}

struct Task {
    name: String,
    description: String,
    status: Status,
}

impl Task {
    fn new(name: &str, description: &str) -> Self {
        if name.is_empty() {
            panic!("name cannot be empty.");
        }

        Self {
            name: name.to_string(),
            description: description.to_string(),
            status: Status::NotYet,
        }
    }

    fn focus_on(&mut self) {
        if self.status != Status::NotYet {
            panic!("this task is already doing or done.");
        }

        self.status = Status::Doing;
    }

    fn end_focus(&mut self) {
        if self.status != Status::Doing {
            panic!("this task is not on going.");
        }

        self.status = Status::Done;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _task = Task::new("example task", "this task is an example.");
    }

    #[test]
    #[should_panic(expected = "name cannot be empty.")]
    fn test_new_name_is_empty() {
        let _task = Task::new("", "this task is an example.");
    }

    #[test]
    fn test_focus_on() {
        let mut task = Task::new("example task", "this task is an example.");
        task.focus_on();
    }

    #[test]
    #[should_panic(expected = "this task is already doing or done.")]
    fn test_already_focus_on() {
        let mut task = Task::new("example task", "this task is an example.");
        task.focus_on();
        task.focus_on(); // this cause a panic.
    }

    #[test]
    fn test_end_focus() {
        let mut task = Task::new("example task", "this task is an example.");
        task.focus_on();
        task.end_focus();
    }

    #[test]
    #[should_panic(expected = "this task is not on going.")]
    fn test_invalid_end_focus() {
        let mut task = Task::new("example task", "this task is an example.");
        task.end_focus(); // this cause a panic.
    }
}
