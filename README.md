# Magitian models

Models to represent git / magitian data structures.
It also provides ways to interact with them and the database.

The [docs](https://docs.rs/magitian_models).

# Some examples

## Commit extraction from the ODB
The ```marker``` should be a commit's ID in the repo that we wish to start the ```RevWalk``` from.
```rust
let repo = git2::Repository::open("../repo").unwrap();
let marker = /* Marker */

let commits = magitian_models::commit::Commit::from_unadded(&repo, marker).unwrap();

for c in commits {
    println!("Commit: {:?}", c);
}
```