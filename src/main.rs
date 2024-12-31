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
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let task = Task::new("example task", "this task is an example.");
    }

    #[test]
    #[should_panic(expected = "name cannot be empty.")]
    fn test_new_name_is_empty() {
        let task = Task::new("", "this task is an example.");
    }
}
