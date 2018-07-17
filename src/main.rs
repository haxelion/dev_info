extern crate git2;

use git2::{Repository, Reference};

fn main() {
    if let Ok(repo) = Repository::discover(".") {
        if let Ok(reference) = repo.head() {
            if let Some(name) = reference.name() {
                println!("{}", name);
            }
        }
    }
}
