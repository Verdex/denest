
pub mod linearize;

use linearize:: { LazyLinearization, Linearizable };

pub struct LazyMatches<'a, T> where T : Linearizable<'a> {
    f : fn(&mut LazyLinearization<'a, T>) -> Option<&'a T>,
    input : LazyLinearization<'a, T>,
}

impl<'a, T> Iterator for LazyMatches<'a, T> where T : Linearizable<'a> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.input)
    }
}

#[macro_export]
macro_rules! pattern {
    ($name:ident : $t:ty = $init:pat) => { // TODO include block
        // TODO isn't there some complication about namespaces with things like Linearizable?
        // I mean I'm probably going to expand into something that may not have the using I need ...
        // can I just assume that they'll include the using?
        //fn $name<'a>( input : &'a impl linearize::Linearizable<'a> ) {
        fn $name<'a>( input : &'a $t ) -> LazyMatches<'a, $t> {

            // TODO also use linearize::Linearizable needs to go in here, but will probably need a prefix (to a library namespace?)
            // Or I can just assume the consume is going to have the use someplace in scope
            let mut blarg = input.lazy_linearization();

            fn run<'a>(input : &mut LazyLinearization<'a, $t>) -> Option<&'a $t> {
                match input.next() {
                    Some($init) => None,
                    _ => None,
                }
            }

            LazyMatches { f : run, input : blarg }
        }
    };
    ($name:ident : $t:ty = $init:pat, $( [ $($var:ident),* ] => $next:pat )*) => {

    };
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::linearize::{ Linearizable, LazyLinearization };

    #[derive(Debug, PartialEq)]
    enum Tree { 
        Node(Box<Tree>, Box<Tree>),
        Leaf(u8),
    }

    impl<'a> linearize::Linearizable<'a> for Tree {
        fn l_next(&'a self) -> Vec<&'a Self> {
            match self {
                Tree::Node(a, b) => vec![a, b],
                Tree::Leaf(_) => vec![],
            }
        }
    }

    fn n(a : Tree, b : Tree) -> Tree {
        Tree::Node(Box::new(a), Box::new(b))
    }

    fn l(v : u8) -> Tree { 
        Tree::Leaf(v)
    }

    #[test]
    fn it_works() {
        pattern!( blarg : Tree = Tree::Leaf(5) );
        blarg(&l(5));
    }
}
