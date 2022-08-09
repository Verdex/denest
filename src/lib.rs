
pub mod linearize;

use linearize::{ LazyLinearization, Linearizable };

pub struct LazyMatches<'a, T, Out> where T : Linearizable<'a> {
    f : fn(&mut LazyLinearization<'a, T>) -> Option<&'a Out>,
    input : LazyLinearization<'a, T>,
}

impl<'a, T, Out> Iterator for LazyMatches<'a, T, Out> where T : Linearizable<'a> {
    type Item = &'a Out;
    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.input)
    }
}

#[macro_export]
macro_rules! pattern {
    ($name:ident : $t:ty => $out:ty = $init:pat, $b:block) => { // TODO include block
        // TODO isn't there some complication about namespaces with things like Linearizable?
        // I mean I'm probably going to expand into something that may not have the using I need ...
        // can I just assume that they'll include the using?
        fn $name<'a>( input : &'a $t ) -> LazyMatches<'a, $t, $out> {

            // TODO also use linearize::Linearizable needs to go in here, but will probably need a prefix (to a library namespace?)
            // Or I can just assume the consume is going to have the use someplace in scope
            let blarg = input.lazy_linearization();

            fn run<'a>(input : &mut LazyLinearization<'a, $t>) -> Option<&'a $out> {
                while let Some(v) = input.next() {
                    match v {
                        $init => { return Some($b); },
                        _ => { },
                    }
                }
                None
            }

            LazyMatches { f : run, input : blarg }
        }
    };
    ($name:ident : $t:ty => $out:ty = $init:pat, $( [$($var:ident),+] => $next:pat ),+, $b:block) => {
        fn $name<'a>( input : &'a $t ) -> LazyMatches<'a, $t, $out> {

            // TODO also use linearize::Linearizable needs to go in here, but will probably need a prefix (to a library namespace?)
            // Or I can just assume the consume is going to have the use someplace in scope
            let blarg = input.lazy_linearization();

            fn run<'a>(input : &mut LazyLinearization<'a, $t>) -> Option<&'a $out> {
                while let Some(v) = input.next() {
                    match v {
                        $init => {
                            $(
                                $(
                                    if let $next = &**$var { // TODO need box version and no box version
                                        return Some($b);
                                    }
                                ),+ // TODO ident rep broken, also is pat rep broken?
                            ),+
                        },
                        _ => { },
                    }
                }
                None
            }

            LazyMatches { f : run, input : blarg }
        }
    };
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::linearize::{ Linearizable, LazyLinearization };

    #[derive(Debug, PartialEq)]
    struct Thing { 
        value : u8
    }

    #[derive(Debug, PartialEq)]
    enum Tree { 
        Node(Box<Tree>, Box<Tree>),
        Leaf(Thing),
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

    fn l(v : Thing) -> Tree { 
        Tree::Leaf(v)
    }

    #[test]
    fn it_works() {

        pattern!( zap : Tree => Thing = Tree::Node(z, zz), [z, zz] => Tree::Leaf(w), {
            w
        });
        pattern!( blarg : Tree => Thing = Tree::Leaf(z), {
            z
        });
        /*for w in blarg(&n(l(Thing { value: 5 }), l( Thing{ value: 6 }))) {
            println!("{:?}", w);
        }*/


        for w in zap(&n(l(Thing { value: 5 }), l( Thing{ value: 6 }))) {
            println!("{:?}", w);
        }
    }
}
