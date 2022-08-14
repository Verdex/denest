
pub struct Lax<'a, T> where T : Linearizable<'a> {
    q : Vec<&'a T>,
}

impl<'a, T> Iterator for Lax<'a, T> where T : Linearizable<'a> {
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

pub struct Paths<'a, T> where T : Linearizable<'a> {
    q : Vec<Vec<&'a T>>,
    result : Vec<&'a T>,
}

impl<'a, T> Iterator for Paths<'a, T> where T : Linearizable<'a> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.q.pop() {
                Some(mut xs) => {
                    match xs.pop() {
                        Some(x) => {
                            self.q.push(xs);
                            let n = x.l_next();
                            self.q.push(n);
                            self.result.push(x);
                        },
                        None => {
                            if self.q.iter().map(|z| z.len()).fold(0, |s, i| s + i) == 0 { 
                                self.q = vec![];
                            }
                            let ret = self.result.clone();
                            self.result.pop();
                            return Some(ret);
                        },
                    }
                }
                None => { return None; },
            }
        }
    }
}

pub trait Linearizable<'a> {
    fn l_next(&'a self) -> Vec<&'a Self>;

    fn to_lax(&'a self) -> Lax<'a, Self> where Self : Sized {
        Lax { q: vec![ self ] }
    }

    fn paths(&'a self) -> Paths<'a, Self> where Self : Sized {
        Paths { q : vec![ vec![ self ] ], result : vec![] }
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
    fn paths_should_return_all_paths() {
        let input = n(n(l(1), l(2)), l(3));

        let output = input.paths().collect::<Vec<_>>();

        assert_eq!( output, 
            vec![ vec![ &n(n(l(1), l(2)), l(3)), &l(3) ]
                , vec![ &n(n(l(1), l(2)), l(3)), &n(l(1), l(2)), &l(2)]
                , vec![ &n(n(l(1), l(2)), l(3)), &n(l(1), l(2)), &l(1)]
                ] );
    }

    #[test]
    fn lax_should_generate_linear_tree() {
        let input = n(n(l(1), l(2)), l(3));

        let output = input.to_lax().collect::<Vec<_>>();

        assert_eq!( output, vec![&n(n(l(1), l(2)), l(3)), &l(3), &n(l(1), l(2)), &l(2), &l(1)] );
    }
}
