use std::cmp::Eq;

pub struct Cacher<T, S> 
where 
    T: Fn(S) -> S,
    S: Eq + Copy + Clone
{
    calculation: T,
    arg: Option<S>,
    value: Option<S>
}

impl<T, S> Cacher<T, S> 
where 
    T: Fn(S) -> S,
    S: Eq + Copy + Clone
{
    pub fn new(calculation: T) -> Cacher<T, S> {
        Cacher {
            calculation,
            arg: None,
            value: None,
        }
    }
    pub fn value(&mut self, arg: S) -> S {
        match self.arg {
            Some(a) => match a == arg {
                true => self.value.unwrap(),
                false => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    self.arg = Some(arg);
                    v
                }
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                self.arg = Some(arg);
                v
            }
        }
    }
}