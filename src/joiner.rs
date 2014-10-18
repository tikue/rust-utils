use std::fmt;
use std::io;
use std::mem;

pub struct Joiner<T> where T: fmt::Show {
    sep: T,
}

impl<T> Joiner<T> where T: fmt::Show {
    pub fn on(sep: T) -> Joiner<T> {
        Joiner {
            sep: sep
        }
    }

    pub fn join<I, T>(&self, mut it: I) -> String 
        where I: Iterator<T>,
              T: fmt::Show {
        match it.next() {
            None => "".to_string(),
            Some(el) => {
                let mem_per_el = mem::size_of::<T>();
                let (lower, upper) = it.size_hint();
                let it_size = upper.unwrap_or(lower);
                let mut w = io::MemWriter::with_capacity(it_size * mem_per_el);        
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
