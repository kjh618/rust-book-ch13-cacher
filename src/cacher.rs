use std::collections::HashMap;

pub struct Cacher<C, P, R>
    where
        C: Fn(P) -> R,
        P: std::hash::Hash + std::cmp::Eq + Copy,
        R: Copy {
    calculation: C,
    cache: HashMap<P, R>
}

impl<C, P, R> Cacher<C, P, R>
    where
        C: Fn(P) -> R,
        P: std::hash::Hash + std::cmp::Eq + Copy,
        R: Copy {
    pub fn new(calculation: C) -> Cacher<C, P, R> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: P) -> R {
        match self.cache.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let calculation = |x| {
            println!("[test1] calculation({}) called!", x);
            x + 1
        };
        let mut cacher = Cacher::new(calculation);
        assert_eq!(cacher.value(1), 2);
        assert_eq!(cacher.value(1), 2);
        assert_eq!(cacher.value(2), 3);
        assert_eq!(cacher.value(1), 2);
        assert_eq!(cacher.value(2), 3);
    }

    #[test]
    fn test2() {
        let calculation = |x: bool| {
            println!("[test2] calculation({}) called!", x);
            !x
        };
        let mut cacher = Cacher::new(calculation);
        assert_eq!(cacher.value(true), false);
        assert_eq!(cacher.value(true), false);
        assert_eq!(cacher.value(false), true);
        assert_eq!(cacher.value(true), false);
        assert_eq!(cacher.value(false), true);
    }
}
