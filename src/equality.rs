pub trait _Equal<A> {
    type R;
}

impl<T> _Equal<T> for T {
    type R = ();
}

/// Eq<A, B> is a type iff A and B are the same type.
pub type Equal<A, B> = <B as _Equal<A>>::R;
