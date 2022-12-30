use crate::bool::{If, _If};
use crate::func::{Apply, _Apply};
use std::marker::PhantomData;

pub struct Nil;
pub struct Cons<H, T> {
    _p: PhantomData<(H, T)>,
}

pub trait _Append<A> {
    type R;
}
impl<A> _Append<A> for Nil {
    type R = Cons<A, Nil>;
}
impl<A, H, T> _Append<A> for Cons<H, T>
where
    T: _Append<A>,
{
    type R = Cons<H, T::R>;
}

pub struct Append;
impl<A, L> _Apply for (Append, (A, L))
where
    L: _Append<A>,
{
    type R = <L as _Append<A>>::R;
}

pub trait _Concat<A> {
    type R;
}
impl<A> _Concat<A> for Nil {
    type R = A;
}
impl<A, H, T> _Concat<A> for Cons<H, T>
where
    T: _Concat<A>,
{
    type R = Cons<H, T::R>;
}

pub struct Concat;
impl<A, B> _Apply for (Concat, (A, B))
where
    A: _Concat<B>,
{
    type R = <A as _Concat<B>>::R;
}

pub trait _Reverse {
    type R;
}
impl _Reverse for Nil {
    type R = Nil;
}
impl<H, T> _Reverse for Cons<H, T>
where
    T: _Reverse,
    T::R: _Append<H>,
{
    type R = Apply<Append, (H, Apply<Reverse, T>)>;
}

pub struct Reverse;
impl<L> _Apply for (Reverse, L)
where
    L: _Reverse,
{
    type R = <L as _Reverse>::R;
}

pub trait _Map<F> {
    type R;
}
impl<F> _Map<F> for Nil {
    type R = Nil;
}
impl<F, H, T> _Map<F> for Cons<H, T>
where
    (F, H): _Apply,
    T: _Map<F>,
{
    type R = Cons<Apply<F, H>, T::R>;
}

pub struct Map;
impl<F, L> _Apply for (Map, (F, L))
where
    L: _Map<F>,
{
    type R = L::R;
}

pub trait _Filter<F> {
    type R;
}
impl<F> _Filter<F> for Nil {
    type R = Nil;
}
impl<F, H, T> _Filter<F> for Cons<H, T>
where
    (F, H): _Apply,
    <(F, H) as _Apply>::R: _If<Cons<H, T::R>, T::R>,
    T: _Filter<F>,
{
    type R = If<Apply<F, H>, Cons<H, T::R>, T::R>;
}

pub struct Filter;
impl<F, L> _Apply for (Filter, (F, L))
where
    L: _Filter<F>,
{
    type R = L::R;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bool::{False, Not, True};
    use crate::equality::Equal;
    use crate::func::Compose;
    use crate::nat::{Add, S, Z};

    #[test]
    fn it_works() {
        type A = Cons<i8, Cons<i16, Cons<i32, Cons<i64, Nil>>>>;
        type B = Cons<i64, Cons<i32, Cons<i16, Cons<i8, Nil>>>>;

        let _: Equal<Apply<Concat, (Nil, Nil)>, Nil>;
        let _: Equal<Apply<Concat, (Cons<i8, Nil>, Nil)>, Cons<i8, Nil>>;
        let _: Equal<Apply<Concat, (Nil, Cons<i8, Nil>)>, Cons<i8, Nil>>;
        let _: Equal<Apply<Concat, (Cons<i8, Nil>, Cons<i16, Nil>)>, Cons<i8, Cons<i16, Nil>>>;
        let _: Equal<
            Apply<Concat, (Cons<i8, Cons<i16, Nil>>, Cons<i32, Cons<i64, Nil>>)>,
            Cons<i8, Cons<i16, Cons<i32, Cons<i64, Nil>>>>,
        >;

        let _: Equal<Apply<Reverse, Nil>, Nil>;
        let _: Equal<Apply<Reverse, A>, B>;
    }

    #[test]
    fn map() {
        type A = Cons<Z, Cons<S<Z>, Cons<S<S<Z>>, Nil>>>;
        type B = Cons<S<Z>, Cons<S<S<Z>>, Cons<S<S<S<Z>>>, Nil>>>;
        type C = Cons<Z, Cons<S<S<Z>>, Cons<S<S<S<S<Z>>>>, Nil>>>;

        struct Inc;
        impl<N> _Apply for (Inc, N) {
            type R = S<N>;
        }

        let _: Equal<Apply<Map, (Inc, A)>, B>;

        struct Double;
        impl<N> _Apply for (Double, N)
        where
            (Add, (N, N)): _Apply,
        {
            type R = Apply<Add, (N, N)>;
        }

        let _: Equal<Apply<Map, (Double, A)>, C>;

        struct Nada;
        impl<N> _Apply for (Nada, N) {
            type R = ();
        }

        let _: Equal<Apply<Map, (Nada, A)>, Cons<(), Cons<(), Cons<(), Nil>>>>;
    }

    #[test]
    fn filter() {
        struct Zero;
        impl _Apply for (Zero, Z) {
            type R = True;
        }
        impl<N> _Apply for (Zero, S<N>) {
            type R = False;
        }

        struct NonZero;
        impl<N> _Apply for (NonZero, N)
        where
            (Zero, N): _Apply,
            (Not, <(Zero, N) as _Apply>::R): _Apply,
        {
            type R = Apply<Compose<Not, Zero>, N>;
        }

        type A = Cons<Z, Cons<S<Z>, Cons<S<S<Z>>, Nil>>>;
        type B = Cons<S<Z>, Cons<S<S<Z>>, Nil>>;
        let _: Equal<Apply<Filter, (NonZero, A)>, B>;
        let _: Equal<Apply<Filter, (Zero, A)>, Cons<Z, Nil>>;
    }
}
