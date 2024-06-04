macro_rules! build_path {
    ( $($params:expr),+ ) => {
        {
            let mut buf = std::path::PathBuf::new();
            $( buf.push($params); )*
            let str:&str = buf.to_str().ok_or_else(|| BookError::new("Failed to build path"))?;
            String::from(str)
        }
    };
}

pub(crate) use build_path;
use crate::book::BookError;
use crate::book::bookresult::BookResult;

pub fn directory_scan(path: &std::path::Path, f: &mut dyn FnMut(&str)->BookResult) -> BookResult<> {
    for entry in std::fs::read_dir(path)? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            directory_scan(&entry_path, f)?;
        } else {
            f(entry_path.to_str().ok_or_else(|| BookError::new("Unexpected invalid path"))?)?;
        }
    }
    Ok(())
}