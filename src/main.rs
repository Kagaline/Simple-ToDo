#[derive(Debug, PartialEq)]
enum Status {
    NotYet,
    Doing,
    Holding,
    Done,
}

struct Task {
    id: u64,
    name: String,
    description: String,
    status: Status,
}

impl Task {
    fn new(id: u64, name: &str, description: &str) -> Self {
        if name.is_empty() {
            panic!("name cannot be empty.");
        }

        Self {
            id,
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

    fn postpone(&mut self) {
        if self.status != Status::Doing {
            panic!("only doing task can postpone.");
        }

        self.status = Status::Holding;
    }

    fn restart(&mut self) {
        if self.status != Status::Holding {
            panic!("only holding task can restart.");
        }

        self.status = Status::Doing;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _task = Task::new(0, "example task", "this task is an example.");
    }

    #[test]
    #[should_panic(expected = "name cannot be empty.")]
    fn test_new_name_is_empty() {
        let _task = Task::new(0, "", "this task is an example.");
    }

    #[test]
    fn test_focus_on() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
    }

    #[test]
    #[should_panic(expected = "this task is already doing or done.")]
    fn test_already_focus_on() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.focus_on(); // this cause a panic.
    }

    #[test]
    fn test_end_focus() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.end_focus();
    }

    #[test]
    #[should_panic(expected = "this task is not on going.")]
    fn test_invalid_end_focus() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.end_focus(); // this cause a panic.
    }

    #[test]
    fn test_postpone() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.postpone();
    }

    #[test]
    #[should_panic(expected = "only doing task can postpone.")]
    fn test_try_postpone_from_notyet() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.postpone(); // NotYet -> Holding;
    }

    #[test]
    #[should_panic(expected = "only doing task can postpone.")]
    fn test_try_postpone_from_done() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.end_focus();
        task.postpone(); // Done -> Holding;
    }

    #[test]
    #[should_panic(expected = "only doing task can postpone.")]
    fn test_try_postpone_from_holding() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.postpone();
        task.postpone(); // Done -> Holding;
    }

    #[test]
    fn test_restart() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.postpone();
        task.restart();
    }

    #[test]
    #[should_panic(expected = "only holding task can restart.")]
    fn test_try_restart_from_notyet() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.restart();
    }

    #[test]
    #[should_panic(expected = "only holding task can restart.")]
    fn test_try_restart_from_done() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.end_focus();
        task.restart();
    }

    #[test]
    #[should_panic(expected = "only holding task can restart.")]
    fn test_try_restart_from_doing() {
        let mut task = Task::new(0, "example task", "this task is an example.");
        task.focus_on();
        task.restart();
    }
}
