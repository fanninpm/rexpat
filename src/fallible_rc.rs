use std::cell::Cell;
use std::collections::TryReserveError;
use std::rc::Rc;

use fallible_collections::FallibleBox;

pub trait FallibleRc<T> {
    fn try_new(value: T) -> Result<Rc<T>, TryReserveError>;
}

struct RcBox<T> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

impl<T> FallibleRc<T> for Rc<T> {
    fn try_new(value: T) -> Result<Self, TryReserveError> {
        let inner = RcBox {
            strong: Cell::new(1),
            weak: Cell::new(1),
            value,
        };
        // FIXME: the Error types should match, but for some reason
        // it now returns the one from the hashbrown crate
        let b = Box::try_new(inner).map_err(|_| TryReserveError::CapacityOverflow)?;
        unsafe {
            // WARNING: really unsafe!!!
            Ok(std::mem::transmute(b))
        }
    }
}
