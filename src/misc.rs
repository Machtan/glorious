use std::fmt::{self, Debug};

pub struct Ellipsis;

impl Debug for Ellipsis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("..")
    }
}
