use crate::{GIT_COMMIT, GIT_URL};
use git2::{AutotagOption, FetchOptions, Repository};
use std::{error::Error, fs, path::Path};

pub fn prepare_git_repo(repo_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
	// try opening if maybe already cloned
	// otherwise clone
	let repo = match Repository::open(&repo_path) {
		Ok(repo) => {
			// Fetch the latest changes from the remote repository
			let mut fo = FetchOptions::new();
			fo.download_tags(AutotagOption::All);

			// Fetch from origin and ensure we have the latest refs
			repo.find_remote("origin")?.fetch(
				&["refs/heads/*:refs/remotes/origin/*"],
				Some(&mut fo),
				None,
			)?;

			{
				// Get the latest commit of the default branch
				let default_branch = repo.find_reference("refs/remotes/origin/master")?;
				let latest_commit = default_branch.peel_to_commit()?;

				// Reset the repository  clearing any local changes
				repo.reset(latest_commit.as_object(), git2::ResetType::Hard, None)?;
			}

			repo
		}
		Err(_) => {
			// Failed to open the repository for some reason
			// Clone it from the remote repository
			if fs::exists(&repo_path)? {
				fs::remove_dir_all(&repo_path)?;
			}

			Repository::clone_recurse(GIT_URL, repo_path)?
		}
	};

	let commit = repo.find_commit_by_prefix(GIT_COMMIT)?;

	repo.checkout_tree(commit.as_object(), None)?;

	Ok(())
}
