use crate::bool::{False, True};
use crate::func::{Apply, _Apply};
use std::marker::PhantomData;

pub struct Z;
pub struct S<N> {
    _p: PhantomData<N>,
}

/// Predecessor function
pub struct Pred;
impl<N> _Apply for (Pred, S<N>) {
    type R = N;
}

pub trait _Pred {
    type R;
}
impl<N> _Pred for S<N> {
    type R = N;
}
pub type PPred<N> = <N as _Pred>::R;

/// Sum of two type numbers
pub struct Add;
impl<N> _Apply for (Add, (N, Z)) {
    type R = N;
}
impl<N, K> _Apply for (Add, (K, S<N>))
where
    (Add, (S<K>, N)): _Apply,
{
    type R = Apply<Add, (S<K>, N)>;
}

/// Product of two type numbers
pub struct Mul;
impl<N> _Apply for (Mul, (N, Z)) {
    type R = Z;
}
impl<N, K> _Apply for (Mul, (K, S<N>))
where
    (Mul, (K, N)): _Apply,
    (Add, (K, <(Mul, (K, N)) as _Apply>::R)): _Apply,
{
    type R = Apply<Add, (K, Apply<Mul, (K, N)>)>;
}

/// Difference of two type numbers
pub struct Sub;
impl<N> _Apply for (Sub, (N, Z)) {
    type R = N;
}
impl<N, K> _Apply for (Sub, (S<K>, S<N>))
where
    (Sub, (K, N)): _Apply,
{
    type R = Apply<Sub, (K, N)>;
}

pub struct Less;
impl<N> _Apply for (Less, (Z, S<N>)) {
    type R = True;
}
impl<N> _Apply for (Less, (S<N>, Z)) {
    type R = False;
}
impl<A, B> _Apply for (Less, (S<A>, S<B>))
where
    (Less, (A, B)): _Apply,
{
    type R = Apply<Less, (A, B)>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equality::*;
    use std::any::type_name;

    #[test]
    fn it_works() {
        assert_eq!(type_name::<Z>(), "typelevel::nat::Z");
        assert_eq!(type_name::<S<Z>>(), "typelevel::nat::S<typelevel::nat::Z>");

        type N1 = S<Z>;
        type N2 = S<S<Z>>;
        type N3 = S<S<S<Z>>>;
        type N4 = S<S<S<S<Z>>>>;
        type N5 = S<S<S<S<S<Z>>>>>;
        type N6 = S<S<S<S<S<S<Z>>>>>>;

        let _: Equal<Apply<Add, (N2, N3)>, N5>;
        let _: Equal<Apply<Mul, (N1, Z)>, Z>;
        let _: Equal<Apply<Mul, (Z, N1)>, Z>;
        let _: Equal<Apply<Mul, (N1, N1)>, N1>;
        let _: Equal<Apply<Mul, (N2, N3)>, N6>;

        let _: Equal<Apply<Sub, (Z, Z)>, Z>;
        let _: Equal<Apply<Sub, (N1, N1)>, Z>;
        let _: Equal<Apply<Sub, (N4, N1)>, N3>;
        let _: Equal<Apply<Sub, (N4, N3)>, N1>;
    }

    #[test]
    fn fib() {
        struct Fib;
        impl _Apply for (Fib, Z) {
            type R = S<Z>;
        }
        impl _Apply for (Fib, S<Z>) {
            type R = S<Z>;
        }
        impl<N> _Apply for (Fib, S<S<N>>)
        where
            (Fib, N): _Apply,
            (Fib, S<N>): _Apply,
            (Add, (<(Fib, N) as _Apply>::R, <(Fib, S<N>) as _Apply>::R)): _Apply,
        {
            type R = Apply<Add, (Apply<Fib, N>, Apply<Fib, S<N>>)>;
        }

        let _: Equal<Apply<Fib, Z>, S<Z>>;
        let _: Equal<Apply<Fib, S<Z>>, S<Z>>;
        let _: Equal<Apply<Fib, S<S<Z>>>, S<S<Z>>>;
        let _: Equal<Apply<Fib, S<S<S<Z>>>>, S<S<S<Z>>>>;
        let _: Equal<Apply<Fib, S<S<S<S<Z>>>>>, S<S<S<S<S<Z>>>>>>;
        let _: Equal<Apply<Fib, S<S<S<S<S<Z>>>>>>, S<S<S<S<S<S<S<S<Z>>>>>>>>>;
        let _: Equal<Apply<Fib, S<S<S<S<S<S<Z>>>>>>>, S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>>;
    }

    #[test]
    fn fact() {
        struct Fact;
        impl _Apply for (Fact, Z) {
            type R = S<Z>;
        }
        impl<N> _Apply for (Fact, S<N>)
        where
            (Fact, N): _Apply,
            (Mul, (<(Fact, N) as _Apply>::R, S<N>)): _Apply,
        {
            type R = Apply<Mul, (Apply<Fact, N>, S<N>)>;
        }

        let _: Equal<Apply<Fact, Z>, S<Z>>;
        let _: Equal<Apply<Fact, S<Z>>, S<Z>>;
        let _: Equal<Apply<Fact, S<S<Z>>>, S<S<Z>>>;
        let _: Equal<Apply<Fact, S<S<S<Z>>>>, S<S<S<S<S<S<Z>>>>>>>;
        let _: Equal<Apply<Fact, S<S<S<S<Z>>>>>, S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>>>>>>>>>>>>>;
    }
}
