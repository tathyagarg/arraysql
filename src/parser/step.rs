use std::fmt;

#[derive(Default, Debug)]
pub enum Step {
    #[default]
    Start,
    DatabaseName,
    End,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Step::Start => write!(f, "Start"),
            Step::DatabaseName => write!(f, "DatabaseName"),
            Step::End => write!(f, "End"),
        }
    }
}
