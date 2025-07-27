// F: First -> Second
// G: Second -> Third
// H: First -> Third
// H = compose_two(F, G)
pub fn compose_two<First, Second, Third, F, G>(f: F, g: G) -> impl Fn(First) -> Third
where
    F: Fn(First) -> Second,
    G: Fn(Second) -> Third,
{
    move |x| g(f(x))
}

#[macro_export]
macro_rules! compose {
    ($f:expr) => {$f};
    ($f:expr, $($fs:expr),+) => {
        compose_two($f, compose!($($fs),+))
    };
}

// macros exported with #[macro_export] are expanded at the crate root, so they cannot see private items
// Use the full path to compose_two in your macro, referencing it from the crate root.
// <code> $crate::utils::funcs::compose_two</code>
#[macro_export]
macro_rules! compose_alt {
    ($f:expr) => {$f};
    ($f:expr, $($fs:expr),+) => {
        $crate::utils::funcs::compose_two($f, compose_alt!($($fs),+))
    };
}


//curry!(|a,b|a+b)
#[macro_export]
macro_rules! curry {
    (|$first_arg: ident $(, $arg: ident)*| $function_body: expr) => {
        move |$first_arg| $(move| $arg|)* {
            $function_body
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compose() {
        let composed = compose!(|x: i32| x + 1, |x: i32| x * 2, |x: i32| x - 3);
        assert_eq!(composed(5), 9); // ((5 + 1) * 2) - 3 = 9
    }

    #[test]
    fn test_curry() {
        let add = curry!(|a, b| a + b);   
        assert_eq!(add(5)(10), 15); // 5 + 10 = 15
    }
}   