use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// An in-database path segment
///
/// Each path in the database needs to be unique in it's scope (user
/// or global).  A path is terminated by the leaf element, also knows
/// as the record-id.  This does however not mean that the path
/// becomes blocked for future writes to add further subseqs.
///
/// - Path A: `/foo/bar:baz`
/// - Path B: `/foo/bar/baz:beep`
///
/// Are both valid and will not collide.
///
/// ## String representation
///
/// An alexandria path can be represented as a string with `:` and `/`
/// acting as delimiters.  Following are some valid examples that can
/// be parsed from strings (or turned into strings).
///
/// - `/msg/private:bob`: msg -> private -> bob
/// - `/msg/private/bob:imgs`: msg -> private -> bob -> imgs
/// - `/sessions:all` -> sessions -> all
///
/// The following code can be used to create Path objects from
/// strings:
///
/// ```rust
/// # use alexandria::utils::Path;
/// let _: Path = "/test:data".into();
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Path {
    leaf: String,
    seq: Vec<String>,
}

impl Path {
    pub fn new<S, I>(leaf: S, seq: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            leaf: leaf.into(),
            seq: seq.into_iter().map(Into::into).collect(),
        }
    }

    pub fn leaf(&self) -> &str {
        self.leaf.as_str()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Path> for String {
    fn from(p: &Path) -> String {
        format!("/{}:{}", p.seq.join("/"), p.leaf)
    }
}

impl From<String> for Path {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'path> From<&'path str> for Path {
    fn from(s: &'path str) -> Self {
        let mut vec: Vec<_> = s.split(":").collect();
        let leaf = vec.remove(1).to_string();
        let seq = vec
            .remove(0)
            .split("/")
            .filter(|seg| seg != &"")
            .map(|s| s.to_string())
            .collect();
        Self { leaf, seq }
    }
}

#[test]
fn parse_path_simple() {
    let path = "/msg:bob";
    let Path { leaf, seq } = path.into();

    assert_eq!(leaf, "bob".to_string());
    assert_eq!(seq, vec!["msg".to_string()]);
}

// #[macro_export]
// macro_rules! path {
//     ($x:expr) => (Path::new($x, vec![]));
//     ($x:expr, $($y:expr),+) => (Path::new($x, vec![ $($y),+ ]));
// }

// #[test]
// fn path_from_macro() {
//     let p = path!("cool", vec!["img", "bob"]);
//     assert_eq!(p, "/img/bob:cool".into());
// }
