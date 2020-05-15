use git2::Repository;
use std::env;
use std::error::Error;

pub fn latest_commit() -> Result<String, Box<dyn Error>> {
    let path = env::current_dir()?;
    let repo = Repository::discover(path)?;
    let revspec = repo.revparse("HEAD")?;
    if revspec.mode().contains(git2::RevparseMode::SINGLE) {
        if let Some(c) = revspec.from() {
            return Ok(c.id().to_string());
        }
    }
    Err("no rev")?
}
