# Either an owned or borrowed value, with type known at compile time

This crate offers a `MaybeOwned` trait which allows you to pass either an owned
or a borrowed value to a function. As opposed to [`std::borrow::Cow`]
and [`beef::Cow`], this trait

* allows you to keep call sites clean, as you don't need to explicitly construct `Cow`s;
* allows you to perform a [static dispatch] without relying on [constant propagation],
if possible;
* consequently benefit from various optimizations (notably, [dead code elimination] and
[constant propagation]), if possible.

However, it also creates additional mental overhead and in case cases might cause
genericity-induced problems.

Eventually, definition sites might become cleaner with an attribute proc macro.

## Simple example

```rust,no_run
use rand::Rng;
use maybe_owned_trait::MaybeOwned;
use std::path::{Path, PathBuf};

// Definition site is a bit more verbose than with `Cow`, yet still reasonable
fn my_fn(path: impl for<'a> MaybeOwned<Owned = PathBuf, Borrowed<'a> = &'a Path>) {
    // Of course, you'll have a meaningful condition here
    if rand::thread_rng().gen::<bool>() {
        let path_buf: PathBuf = path.to_owned();
        println!("Owned buf: {:?}", path_buf);
    } else {
        let path: &Path = path.borrow();
        println!("Borrowed path: {:?}", path);
    };
}

let path = PathBuf::from("hello");
// Call sites are clean
my_fn(&path);
my_fn(path);
```

[static dispatch]: https://en.wikipedia.org/wiki/Static_dispatch
[dead code elimination]: https://en.wikipedia.org/wiki/Dead-code_elimination
[constant propagation]: https://en.wikipedia.org/wiki/Constant_folding
[`std::borrow::Cow`]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html
[`beef::Cow`]: https://docs.rs/beef/latest/beef/type.Cow.html
