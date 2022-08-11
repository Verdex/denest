
pub mod linearize;

use linearize::{ LazyLinearization, Linearizable };

pub struct LazyMatches<'a, T, Out> where T : Linearizable<'a> {
    f : fn(&mut LazyLinearization<'a, T>) -> Vec<&'a Out>,
    input : LazyLinearization<'a, T>,
    q : Vec<&'a Out>,
}

impl<'a, T, Out> Iterator for LazyMatches<'a, T, Out> where T : Linearizable<'a> {
    type Item = &'a Out;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.q.pop() {
            Some(v)
        }
        else {
            let mut results = (self.f)(&mut self.input);
            self.q.append(&mut results);
            if let Some(v) = self.q.pop() {
                Some(v)
            } 
            else {
                None
            }
        }
    }
}



#[cfg(test)]
mod tests {

}
