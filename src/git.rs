use git2::{DiffOptions, Repository, StatusOptions};
use crate::{Error, Result, GitConfig};

pub struct GitRepo {
    repo: Repository,
    config: GitConfig,
}

impl GitRepo {
    pub fn new(config: GitConfig) -> Result<Self> {
        Ok(Self {
            repo: Repository::open(&config.repo_path)?,
            config,
        })
    }

    pub fn get_diff(&self) -> Result<String> {
        let mut diff_options = DiffOptions::new();
        diff_options.include_untracked(self.config.include_untracked);
        
        let diff = if self.is_initial_commit()? {
            // For initial commits, diff against an empty tree
            let empty_tree = self.repo.find_tree(self.repo.treebuilder(None)?.write()?)?;
            let mut index = self.repo.index()?;
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;
            let tree_id = index.write_tree()?;
            let tree = self.repo.find_tree(tree_id)?;
            self.repo.diff_tree_to_tree(Some(&empty_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            // For normal commits, diff against the index
            self.repo.diff_index_to_workdir(None, Some(&mut diff_options))?
        };
        
        let mut diff_string = String::new();
        diff.print(git2::DiffFormat::Patch, |_, _, line| {
            diff_string.push_str(&String::from_utf8_lossy(line.content()));
            true
        })?;
        
        if diff_string.is_empty() {
            return Err(Error::NoChanges);
        }
        
        Ok(diff_string)
    }

    fn is_initial_commit(&self) -> Result<bool> {
        Ok(self.repo.head().is_err())
    }

    pub fn has_changes(&self) -> Result<bool> {
        let mut status_options = StatusOptions::new();
        status_options.include_untracked(self.config.include_untracked);
        
        let statuses = self.repo.statuses(Some(&mut status_options))?;
        Ok(!statuses.is_empty())
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        // First stage all changes
        self.stage_all()?;

        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let signature = self.repo.signature()?;
        let parent = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents = parent.as_ref().map(|p| vec![p]).unwrap_or_default();
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            parents.as_slice(),
        )?;

        Ok(())
    }

    pub fn stage_all(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }
} 