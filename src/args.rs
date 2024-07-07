use std::path::PathBuf;

#[derive(Clone)]
pub struct IncludedTemplate {
    pub name: String,
    pub path: PathBuf,
}

impl std::str::FromStr for IncludedTemplate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.splitn(2, '=').collect::<Vec<&str>>().as_slice() {
            [name, path] => match PathBuf::from_str(path) {
                Ok(path) => Ok(IncludedTemplate {
                    name: name.to_string(),
                    path,
                }),
                Err(e) => Err(format!("Cannot parse file path: {}", e)),
            },
            _ => Err(format!(
                "Cannot determine name and path for included template: {}",
                s
            )),
        }
    }
}

impl std::fmt::Display for IncludedTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.path.display())
    }
}
