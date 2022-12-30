use crate::func::{Apply, _Apply};

pub struct True;
pub struct False;

pub struct Not;

impl _Apply for (Not, True) {
    type R = False;
}
impl _Apply for (Not, False) {
    type R = True;
}

pub struct And;

impl _Apply for (And, (True, True)) {
    type R = True;
}
impl _Apply for (And, (False, True)) {
    type R = False;
}
impl _Apply for (And, (True, False)) {
    type R = False;
}
impl _Apply for (And, (False, False)) {
    type R = False;
}

pub struct Or;

impl<A, B> _Apply for (Or, (A, B))
where
    (Not, A): _Apply,
    (Not, B): _Apply,
    (And, (<(Not, A) as _Apply>::R, <(Not, B) as _Apply>::R)): _Apply,
    (
        Not,
        <(And, (<(Not, A) as _Apply>::R, <(Not, B) as _Apply>::R)) as _Apply>::R,
    ): _Apply,
{
    type R = Apply<Not, Apply<And, (Apply<Not, A>, Apply<Not, B>)>>;
}

pub struct Xor;
impl<A, B> _Apply for (Xor, (A, B))
where
    (Or, (A, B)): _Apply,
    (And, (A, B)): _Apply,
    (Not, <(And, (A, B)) as _Apply>::R): _Apply,
    (
        And,
        (
            <(Or, (A, B)) as _Apply>::R,
            <(Not, <(And, (A, B)) as _Apply>::R) as _Apply>::R,
        ),
    ): _Apply,
{
    type R = Apply<And, (Apply<Or, (A, B)>, Apply<Not, Apply<And, (A, B)>>)>;
}

pub trait _If<A, B> {
    type R;
}
impl<A, B> _If<A, B> for True {
    type R = A;
}
impl<A, B> _If<A, B> for False {
    type R = B;
}
pub type If<C, A, B> = <C as _If<A, B>>::R;

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn it_works() {
        assert_eq!(
            type_name::<Apply<Or, (True, True)>>(),
            "typelevel::bool::True"
        );
        assert_eq!(
            type_name::<Apply<Or, (False, True)>>(),
            "typelevel::bool::True"
        );
        assert_eq!(
            type_name::<Apply<Or, (True, False)>>(),
            "typelevel::bool::True"
        );
        assert_eq!(
            type_name::<Apply<Or, (False, False)>>(),
            "typelevel::bool::False"
        );
    }
}
