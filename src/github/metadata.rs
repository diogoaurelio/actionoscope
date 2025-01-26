use git2::Repository;
use std::collections::HashMap;

pub fn get_git_repo_vars() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let repo = Repository::open(".")
        .expect("unable to read git metadata - are you located at the repository root?");
    let config = repo.config()?;

    let mut vars = HashMap::new();
    if let Ok(remote_url) = config.get_string("remote.origin.url") {
        let parts: Vec<&str> = if remote_url.starts_with("git@") {
            remote_url
                .trim_start_matches("git@")
                .split(':')
                .collect::<Vec<&str>>()[1]
                .split('/')
                .collect()
        } else {
            remote_url.split('/').collect()
        };
        if parts.len() >= 2 {
            let owner = parts[parts.len() - 2];
            let repo_name = parts[parts.len() - 1].trim_end_matches(".git");
            vars.insert("repository_owner".to_string(), owner.to_string());
            vars.insert("repository".to_string(), repo_name.to_string());
        }
    }

    if let Ok(branch) = repo.head() {
        if let Some(name) = branch.shorthand() {
            vars.insert("ref_name".to_string(), name.to_string());
        }
    }

    Ok(vars)
}
