use git2::Repository;
use std::process::Command;
use std::path::Path;
use std::fs;

fn check_for_updates(repo_url: &str, repo_dir: &str) {
    // Clone the repository if it doesn't exist
    if !Path::new(repo_dir).exists() {
        println!("Cloning repository...");
        let _ = Repository::clone(repo_url, repo_dir).expect("Failed to clone repository");
    } else {
        // Open the existing repository
        let repo = Repository::open(repo_dir).expect("Failed to open repository");

        // Fetch the latest changes
        let mut remote = repo.find_remote("origin").expect("Failed to find remote");
        remote.fetch(&["main"], None, None).expect("Failed to fetch from remote");

        // Get the latest commit in the local repository
        let local_commit = repo.head().unwrap().peel_to_commit().unwrap();
        let local_commit_id = local_commit.id();

        // Get the latest commit in the remote repository
        let remote_commit = repo.find_reference("refs/remotes/origin/main").unwrap().peel_to_commit().unwrap();
        let remote_commit_id = remote_commit.id();

        // Compare the two commits
        if local_commit_id != remote_commit_id {
            println!("Updates found. Pulling the latest changes...");
            repo.find_reference("refs/heads/main").unwrap().set_target(remote_commit_id, "Updating to latest commit").unwrap();

            let mut checkout_builder = git2::build::CheckoutBuilder::new();
            repo.checkout_head(Some(&mut checkout_builder)).expect("Failed to checkout head");

            println!("Changes applied. Recompiling the application...");

            // Run the command to recompile the application
            Command::new("cargo")
                .arg("build")
                .status()
                .expect("Failed to recompile the application");
        } else {
            println!("No updates found.");
        }
    }
}

fn main() {
    let repo_url = "https://github.com/ch-love/gemini";
    let repo_dir = "./gemini_repo";

    // Check for updates when the application starts
    check_for_updates(repo_url, repo_dir);

    // Your application logic here
    println!("Application started...");
}
