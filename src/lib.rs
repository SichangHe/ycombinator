fn leak_ref<T>(x: T) -> &'static mut T {
    let b = Box::new(x);
    Box::leak(b)
}

struct P<F>(&'static dyn Fn(&'static P<F>) -> F)
where
    F: 'static;

impl<F> P<F>
where
    F: 'static,
{
    fn call(&self, other: &'static Self) -> F {
        self.0(other)
    }
}

#[allow(clippy::redundant_closure_call)]
pub fn y<X, F, T>(x: &'static X) -> F
where
    X: Fn(&'static dyn Fn(T) -> T) -> F,
    F: 'static + Fn(T) -> T,
    T: 'static,
{
    (|proc: &'static P<F>| x(leak_ref(move |arg: T| (proc.call(proc))(arg))))(leak_ref(P(
        leak_ref(move |proc: &'static P<F>| x(leak_ref(move |arg: T| (proc.call(proc))(arg)))),
    )))
}

pub fn f(func_arg: &dyn Fn(usize) -> usize) -> impl '_ + Fn(usize) -> usize {
    move |n| if n == 0 { 1 } else { n * func_arg(n - 1) }
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
}
