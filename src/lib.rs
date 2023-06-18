//! # Y combinator

use list::Cons;

pub mod list;

/// Heap allocate `x` and return a reference to it.
/// Never deallocate the memory.
pub fn leak<T>(x: T) -> &'static mut T {
    let b = Box::new(x);
    Box::leak(b)
}

struct P<F: 'static>(&'static dyn Fn(&'static P<F>) -> F);

impl<F> P<F> {
    fn call(&self, other: &'static Self) -> F {
        self.0(other)
    }
}

/// # The Y combinator
///
/// The passed-in function `x` has to be `'static`, which means that they could
/// either be static functions declared in the global scope or closures that are
/// leaked using `Box::leak`.
///
/// ## Limitations
///
/// - This function makes 4 heap allocations of closures and never deallocates
///     (therefore leaks) them.
/// - The passed-in function `x` must take `'static` input `I` and `'static`
///     output `O`.
#[allow(clippy::redundant_closure_call)]
pub fn y<X, F, I, O>(x: &'static X) -> F
where
    X: Fn(&'static dyn Fn(I) -> O) -> F,
    F: 'static + Fn(I) -> O,
    I: 'static,
    O: 'static,
{
    (|proc: &'static P<F>| x(leak(move |arg| (proc.call(proc))(arg))))(leak(P(leak(
        move |proc: &'static P<F>| x(leak(move |arg| (proc.call(proc))(arg))),
    ))))
}

/// Function used to create the factorial function using the Y combinator.
pub fn f(func_arg: &dyn Fn(usize) -> usize) -> impl '_ + Fn(usize) -> usize {
    move |n| if n == 0 { 1 } else { n * func_arg(n - 1) }
}

pub fn m(
    func_arg: &dyn Fn(Option<Cons<usize>>) -> Option<usize>,
) -> impl '_ + Fn(Option<Cons<usize>>) -> Option<usize> {
    move |l| match l {
        None => None,
        Some(l) => match l.cdr {
            None => Some(l.car),
            Some(r) => match l.car < r.car {
                true => func_arg(Some(*r)),
                false => func_arg(Some(Cons {
                    car: l.car,
                    cdr: r.cdr,
                })),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial() {
        let fact = y(&f);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(5), 120);
        assert_eq!(fact(10), 3628800);
    }

    #[test]
    fn max() {
        let find_max = y(&m);
        assert_eq!(find_max(Cons::from([1, 2, 3, 4])), Some(4));
        assert_eq!(find_max(Cons::from([4, 2, 1, 3])), Some(4));
        assert_eq!(find_max(Cons::from([3])), Some(3));
        assert_eq!(find_max(Cons::from([0, 0, 0])), Some(0));
        assert_eq!(find_max(Cons::from([1, 0, 1])), Some(1))
    }
}
