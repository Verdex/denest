
pub mod linearize;

use linearize::Linearizable;

macro_rules! pattern {
    ($name:ident : $t:ty = $init:pat) => { // TODO include block
        // TODO isn't there some complication about namespaces with things like Linearizable?
        // I mean I'm probably going to expand into something that may not have the using I need ...
        // can I just assume that they'll include the using?
        //fn $name<'a>( input : &'a impl linearize::Linearizable<'a> ) {
        fn $name( input : &$t ) {

            // TODO also use linearize::Linearizable needs to go in here, but will probably need a prefix (to a library namespace?)
            // Or I can just assume the consume is going to have the use someplace in scope
            let mut blarg = input.lazy_linearization();

            for z in blarg {
                match z {
                    $init => { },
                    _ => { },
                }
            }
        }
    };
    ($name:ident : $t:ty = $init:pat, $( [ $($var:ident),* ] => $next:pat )*) => {

    };
}


#[cfg(test)]
mod tests {

    use super::*;

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
