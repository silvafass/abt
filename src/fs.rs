use std::{fs::ReadDir, path::PathBuf};

pub struct FileCrawler {
    dirs: Vec<PathBuf>,
    files: Option<ReadDir>,
}

impl From<&PathBuf> for FileCrawler {
    fn from(path: &PathBuf) -> Self {
        return FileCrawler {
            dirs: vec![path.to_owned()],
            files: None,
        };
    }
}

impl Iterator for FileCrawler {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(read_dir) = &mut self.files {
                match read_dir.next() {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if path.is_dir() {
                            self.dirs.push(path.clone());
                            continue;
                        }
                        return Some(path);
                    }
                    None => {
                        self.files = None;
                        break;
                    }
                    _ => (),
                }
            }
            while let Some(dir) = self.dirs.pop() {
                match dir.read_dir() {
                    Ok(files) => {
                        self.files = Some(files);
                        return Some(dir);
                    }
                    Err(err) => println!("{} - {}", err, dir.display()),
                }
            }
            break;
        }
        return None;
    }
}
