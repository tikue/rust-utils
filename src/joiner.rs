use std::fmt;
use std::io;

/// An object which joins elements that implement Show (specified as an Iterator) with a separator.
pub struct Joiner<T> where T: fmt::Show {
    sep: T,
}

impl<T> Joiner<T> where T: fmt::Show {
    /// Returns a Joiner that automatically places `sep` between consecutive elements.
    pub fn on(sep: T) -> Joiner<T> {
        Joiner {
            sep: sep
        }
    }

    /// Returns a String containing the String representation of each element in `it`, using the previously configured
    /// separator between each.
    pub fn join<I, T>(&self, mut it: I) -> String 
        where I: Iterator<T>,
              T: fmt::Show {
        match it.next() {
            None => "".to_string(),
            Some(el) => {
                // Create a MemWriter that will be used to store the intermediate results of writing
                // each element from the iterator
                let mut w = io::MemWriter::new();

                // We don't want a separator before the first element
                (write!(&mut w, "{}", el)).ok().unwrap();
                for el in it {
                    (write!(&mut w, "{}{}", self.sep, el)).ok().unwrap();
                }
                String::from_utf8(w.unwrap()).ok().unwrap()
            }
        }
    }
}

#[test]
fn test_vec_repr() {
    let vec = vec![0i, 1, 2];
    assert!(Joiner::on("->").join(vec.into_iter()).as_slice() == "0->1->2")
}
