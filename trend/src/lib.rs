pub mod raw;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Status {
    New,
    Reading,
    ToDo,
    Done,
    Archive,
}
