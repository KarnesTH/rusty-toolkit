use indicatif::{ProgressBar, ProgressStyle};
use inquire::Text;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileSearch {
    pub path: Option<String>,
    pub name: Option<String>,
    pub result: Vec<String>,
}

impl FileSearch {
    /// Create a new instance of the `FileSearch` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - An optional `String` that represents the path to search for files in.
    /// * `name` - An optional `String` that represents the name of the file to search for.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `FileSearch` instance or an error.
    pub fn new(
        path: Option<String>,
        name: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            path,
            name,
            result: vec![],
        })
    }

    /// Run the file search command.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an error.
    ///
    /// # Errors
    ///
    /// This method will return an error if the user input fails.
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = if let Some(path) = self.path.clone() {
            path
        } else {
            Text::new("Enter the path to search for files in:").prompt()?
        };

        let name = if let Some(name) = self.name.clone() {
            name
        } else {
            Text::new("Enter the name of the file to search for:").prompt()?
        };

        let progress = ProgressBar::new_spinner();
        progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} Searching... Found: {pos} files")?,
        );

        self.search(path, &name, progress.clone())?;

        progress.finish_with_message(format!("Found {} files:", self.result.len()));
        for file in &self.result {
            println!("  • {}", file);
        }

        Ok(())
    }

    /// Search for files in a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A `String` that represents the path to search for files in.
    /// * `name` - A reference to a `String` that represents the name of the file to search for.
    /// * `progress` - A `ProgressBar` instance to update the progress of the search.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of files found or an error.
    ///
    /// # Errors
    ///
    /// This method will return an error if the search fails.
    fn search(
        &mut self,
        path: String,
        name: &String,
        progress: ProgressBar,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        for entry in PathBuf::from(path).read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if path.is_dir() {
                    self.search(path.to_string_lossy().to_string(), name, progress.clone())?;
                } else if file_name.contains(name) {
                    self.result.push(path.to_string_lossy().to_string());
                    progress.inc(1);
                }
            }
        }

        Ok(self.result.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file_search = FileSearch::new(None, None).unwrap();
        assert_eq!(file_search.path, None);
        assert_eq!(file_search.name, None);
        assert_eq!(file_search.result, Vec::<String>::new());
    }

    #[test]
    fn test_search() {
        let mut file_search = FileSearch::new(None, None).unwrap();
        assert!(file_search
            .search(
                ".".to_string(),
                &String::from("Cargo"),
                ProgressBar::new_spinner()
            )
            .is_ok());
    }
}
