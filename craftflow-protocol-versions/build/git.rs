use crate::{GIT_COMMIT, GIT_URL};
use git2::{AutotagOption, FetchOptions, Repository};
use std::{fs, path::Path};

pub fn prepare_git_repo(repo_path: impl AsRef<Path>) {
	// try opening if maybe already cloned
	// otherwise clone
	let repo = match Repository::open(&repo_path) {
		Ok(repo) => {
			// Fetch the latest changes from the remote repository
			let mut fo = FetchOptions::new();
			fo.download_tags(AutotagOption::All);

			println!("cargo:warning=Fetching latest changes from the remote repository");

			// Fetch from origin and ensure we have the latest refs
			repo.find_remote("origin")
				.unwrap()
				.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fo), None)
				.unwrap();

			{
				// Get the latest commit of the default branch
				let default_branch = repo.find_reference("refs/remotes/origin/master").unwrap();
				let latest_commit = default_branch.peel_to_commit().unwrap();

				// Reset the repository  clearing any local changes
				repo.reset(latest_commit.as_object(), git2::ResetType::Hard, None)
					.unwrap();
			}

			repo
		}
		Err(_) => {
			// Failed to open the repository for some reason
			// Clone it from the remote repository
			if fs::exists(&repo_path).unwrap() {
				fs::remove_dir_all(&repo_path).unwrap();
			}

			println!("cargo:warning=Cloning the remote repository");

			Repository::clone_recurse(GIT_URL, repo_path).unwrap()
		}
	};

	let commit = repo.find_commit_by_prefix(GIT_COMMIT).unwrap();

	repo.checkout_tree(commit.as_object(), None).unwrap();
}
