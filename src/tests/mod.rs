#[cfg(test)]
mod tests {
    use crate::Args;
    use std::path::PathBuf;

    #[test]
    fn test_get_dir() {
        let args: Args = Args {
            width: 10,
            height: 10,
            output: PathBuf::new(),
            name: String::from("test-empty"),
            path: String::from("test-empty"),
        };

        assert!(args.get_dir().is_err());
    }
    #[test]
    fn test_get_dir_empty_folder() {
        std::fs::create_dir_all("test-empty").unwrap();

        let args = Args {
            width: 10,
            height: 10,
            output: PathBuf::new(),
            name: String::from("test-empty"),
            path: String::from("test-empty"),
        };

        assert!(args.get_dir().is_err());
        std::fs::remove_dir("test-empty").unwrap();
    }
}
