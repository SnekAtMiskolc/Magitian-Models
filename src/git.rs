use git2::{Commit, Repository};

pub mod commit;
pub mod repository;

/// Wrap a ```git2::Commit<'static>``` as a ```RawCommit```.
pub type RawCommit = Commit<'static>;

/// Wrap a ```git2::Repository``` as a ```RawRepository```.
pub type RawRepository = Repository;
