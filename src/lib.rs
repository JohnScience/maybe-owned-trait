#![doc = include_str!("../README.md")]

/// Either an owned or a borrowed value.
pub trait MaybeOwned {
    /// The owned type.
    type Owned;
    /// The borrowed type.
    type Borrowed<'a>
    where
        Self: 'a;
    /// Convert to owned type. A no-op if already owned.
    fn to_owned(self) -> Self::Owned;
    /// Borrow as borrowed type.
    fn borrow(&self) -> Self::Borrowed<'_>;
}

impl<'s> MaybeOwned for &'s str {
    type Owned = String;
    type Borrowed<'a> = &'a str
    where
        's: 'a;
    fn to_owned(self) -> Self::Owned {
        self.to_string()
    }
    fn borrow(&self) -> Self::Borrowed<'_> {
        self
    }
}

impl MaybeOwned for String {
    type Owned = String;
    type Borrowed<'a> = &'a str;
    fn to_owned(self) -> Self::Owned {
        self
    }
    fn borrow(&self) -> Self::Borrowed<'_> {
        self.as_str()
    }
}

impl<'s> MaybeOwned for std::borrow::Cow<'s, str> {
    type Owned = String;
    type Borrowed<'a> = &'a str
    where
        's: 'a;
    fn to_owned(self) -> <Self as MaybeOwned>::Owned {
        self.into_owned()
    }
    fn borrow(&self) -> <Self as MaybeOwned>::Borrowed<'_> {
        self.as_ref()
    }
}

#[cfg(feature = "beef")]
impl<'s> MaybeOwned for beef::Cow<'s, str> {
    type Owned = String;
    type Borrowed<'a> = &'a str
    where
        's: 'a;
    fn to_owned(self) -> <Self as MaybeOwned>::Owned {
        self.into_owned()
    }
    fn borrow(&self) -> <Self as MaybeOwned>::Borrowed<'_> {
        self.as_ref()
    }
}

#[cfg(all(feature = "beef", target_pointer_width = "64"))]
impl<'s> MaybeOwned for beef::lean::Cow<'s, str> {
    type Owned = String;
    type Borrowed<'a> = &'a str
    where
        's: 'a;
    fn to_owned(self) -> <Self as MaybeOwned>::Owned {
        self.into_owned()
    }
    fn borrow(&self) -> <Self as MaybeOwned>::Borrowed<'_> {
        self.as_ref()
    }
}

impl<'a, T: Clone> MaybeOwned for &'a [T] {
    type Owned = Vec<T>;
    type Borrowed<'b> = &'b [T]
    where
        T: 'b,
        Self: 'b;

    fn to_owned(self) -> Self::Owned {
        self.to_vec()
    }

    fn borrow(&self) -> Self::Borrowed<'_> {
        self
    }
}

impl<T> MaybeOwned for Vec<T> {
    type Owned = Vec<T>;
    type Borrowed<'a> = &'a [T]
    where T: 'a;

    fn to_owned(self) -> Self::Owned {
        self
    }

    fn borrow(&self) -> Self::Borrowed<'_> {
        self.as_slice()
    }
}

impl<'s> MaybeOwned for &'s std::path::Path {
    type Owned = std::path::PathBuf;
    type Borrowed<'a> = &'a std::path::Path
    where
        's: 'a;

    fn to_owned(self) -> Self::Owned {
        self.to_path_buf()
    }

    fn borrow(&self) -> Self::Borrowed<'_> {
        self
    }
}

impl MaybeOwned for std::path::PathBuf {
    type Owned = std::path::PathBuf;
    type Borrowed<'a> = &'a std::path::Path;

    fn to_owned(self) -> Self::Owned {
        self
    }

    fn borrow(&self) -> Self::Borrowed<'_> {
        self.as_path()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::path::{Path, PathBuf};

    use super::MaybeOwned;

    #[test]
    #[ignore = "This test exists to ensure that the code compiles"]
    fn simple_test() {
        fn my_fn(path: impl MaybeOwned<Owned = PathBuf>) {
            let path = path.to_owned();
            println!("{:?}", path);
        }

        my_fn(PathBuf::from("hello"));
    }

    #[test]
    #[ignore = "This test exists to ensure that the code compiles"]
    fn complicated_test() {
        fn my_fn(path: impl for<'a> MaybeOwned<Owned = PathBuf, Borrowed<'a> = &'a Path>) {
            if rand::thread_rng().gen::<bool>() {
                let path_buf: PathBuf = path.to_owned();
                println!("Owned buf: {:?}", path_buf);
            } else {
                let path: &Path = path.borrow();
                println!("Borrowed path: {:?}", path);
            };
        }

        my_fn(PathBuf::from("hello"));
        my_fn(Path::new("hello"));
    }
}
