/// (Higher order) type-level functions
use std::marker::PhantomData;

/// Apply a type function to a type value
pub trait _Apply {
    type R;
}

pub type Apply<F, A> = <(F, A) as _Apply>::R;

/// Apply type function A to the result of B
pub struct Compose<A, B> {
    _p: PhantomData<(A, B)>,
}

impl<A, B, X> _Apply for (Compose<A, B>, X)
where
    (B, X): _Apply,
    (A, <(B, X) as _Apply>::R): _Apply,
{
    type R = Apply<A, Apply<B, X>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equality::Equal;
    use crate::nat::{Add, S, Z};

    #[test]
    fn it_works() {
        struct Inc;
        impl<N> _Apply for (Inc, N) {
            type R = S<N>;
        }

        let _: Equal<Apply<Inc, Z>, S<Z>>;
    }

    #[test]
    fn complexdef() {
        trait Double {
            type R;
        }
        impl<N> Double for N
        where
            (Add, (N, N)): _Apply,
        {
            type R = Apply<Add, (N, N)>;
        }

        trait ThreeTimes {
            type R;
        }
        impl<N> ThreeTimes for N
        where
            N: Double,
            (Add, (N::R, N)): _Apply,
        {
            type R = Apply<Add, (N::R, N)>;
        }

        trait FiveTimes {
            type R;
        }
        impl<N> FiveTimes for N
        where
            N: Double,
            N: ThreeTimes,
            (Add, (<N as Double>::R, <N as ThreeTimes>::R)): _Apply,
        {
            //type R = <<N as ThreeTimes>::R as _Add<<N as Double>::R>>::R;
            type R = Apply<Add, (<N as Double>::R, <N as ThreeTimes>::R)>;
        }

        //define!((fivetimes n): (_Add (Double n) (ThreeTimes n)));

        //define!((fourtimes n): (_Add (Double n) (Double n)));

        //let _: Equal<Apply<fourtimes, S<Z>>, S<S<Z>>>;
    }
}
