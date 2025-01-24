use indicatif::ProgressBar;
use inquire::Text;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileSearch {
    pub path: Option<String>,
    pub name: Option<String>,
    pub result: Vec<String>,
}

impl FileSearch {
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

        let progress = ProgressBar::new(0);
        let founded_files = self.search(path, &name, progress.clone())?;
        progress.set_length(founded_files);
        for file in self.result.iter() {
            progress.println(file);
        }
        progress.finish();

        Ok(())
    }

    pub fn search(
        &mut self,
        path: String,
        name: &String,
        progress: ProgressBar,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        for entry in PathBuf::from(path).read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if path.is_dir() {
                self.search(path.to_str().unwrap().to_string(), name, progress.clone())?;
            } else if file_name.contains(name) {
                self.result.push(path.to_str().unwrap().to_string());
                progress.inc(1);
            }
        }

        Ok(self.result.len() as u64)
    }
}
