
pub struct LazyLinearization<'a, T> {
    q : Vec<&'a T>,
}

impl<'a, T> Iterator for LazyLinearization<'a, T> where T : Linearizable<'a> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.q.pop() {
            Some(x) => {
                let mut nexts = x.l_next();
                self.q.append(&mut nexts);
                Some(x) 
            },
            None => None,
        }
    }
}

pub trait Linearizable<'a> {
    fn l_next(&'a self) -> Vec<&'a Self>;

    fn lazy_linearization(&'a self) -> LazyLinearization<'a, Self> where Self : Sized{
        LazyLinearization { q: vec![ self ] }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq)]
    enum Tree { 
        Node(Box<Tree>, Box<Tree>),
        Leaf(u8),
    }

    impl<'a> Linearizable<'a> for Tree {
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
    fn lazy_linearization_should_generate_linear_tree() {
        let input = n(n(l(1), l(2)), l(3));

        let output = input.lazy_linearization().collect::<Vec<_>>();

        assert_eq!( output, vec![&n(n(l(1), l(2)), l(3)), &l(3), &n(l(1), l(2)), &l(2), &l(1)] );
    }
}