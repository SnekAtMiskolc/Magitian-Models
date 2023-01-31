use git2::Commit;

pub mod commit;

/// Wrap a ```git2::Commit<'static>``` as a ```RawCommit```.
pub type RawCommit = Commit<'static>;
