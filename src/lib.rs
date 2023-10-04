
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

pub struct LaxCut<'a, 'b, T> where T : Linearizable<'a> {
    q : Vec<&'a T>,
    f : &'b dyn Fn(&'a T) -> bool,
}

impl<'a, 'b, T> Iterator for LaxCut<'a, 'b, T> where T : Linearizable<'a> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.q.pop() {
            if (self.f)(x) {
                let mut nexts = x.l_next();
                self.q.append(&mut nexts);
                return Some(x);
            }
        }
        None
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

    fn to_lax_cut<'b, F : Fn(&Self) -> bool>(&'a self, f : &'b F) -> LaxCut<'a, 'b, Self> where Self : Sized {
        LaxCut { q: vec![ self ], f }
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
        SNode(Box<Tree>),
        Node(Box<Tree>, Box<Tree>),
        Leaf(u8),
    }

    impl<'a> Linearizable<'a> for Tree {
        fn l_next(&'a self) -> Vec<&'a Self> {
            match self {
                Tree::SNode(a) => vec![a],
                Tree::Node(a, b) => vec![a, b],
                Tree::Leaf(_) => vec![],
            }
        }
    }

    fn s(a : Tree) -> Tree {
        Tree::SNode(Box::new(a))
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

    #[test]
    fn lax_cut_should_generate_linear_tree_with_cut_subtrees() {
        let input = n(n(s(n(l(0), l(1))), l(2)), n(l(3), l(4)));

        let output = input.to_lax_cut(&mut |x| !matches!(x, Tree::SNode(_))).collect::<Vec<_>>();

        assert_eq!( output, vec![ &n(n(s(n(l(0), l(1))), l(2)), n(l(3), l(4)))
                                , &n(l(3), l(4))
                                , &l(4)
                                , &l(3)
                                , &n(s(n(l(0), l(1))), l(2))
                                , &l(2)
                                ] );
    }

    #[test]
    fn lax_cut_should_be_able_to_return() {
        let input = n(n(s(n(l(0), l(1))), l(2)), n(l(3), l(4)));

        fn t<'a>(input : &'a Tree) -> impl Iterator<Item = &'a Tree> {
            input.to_lax_cut(& |x| !matches!(x, Tree::SNode(_)))
        }

        let output = t(&input).collect::<Vec<_>>();

        assert_eq!( output, vec![ &n(n(s(n(l(0), l(1))), l(2)), n(l(3), l(4)))
                                , &n(l(3), l(4))
                                , &l(4)
                                , &l(3)
                                , &n(s(n(l(0), l(1))), l(2))
                                , &l(2)
                                ] );
    }

    #[test]
    fn lax_cut_should_be_able_to_accept_moved_predicate() {
        let input = n(n(l(1), l(2)), l(3));

        fn t<'a>(input : &'a Tree) -> impl Iterator<Item = &'a Tree> {
            let target = l(2);
            input.to_lax_cut(& move |x| *x != l(2))
        }

        let output = t(&input).collect::<Vec<_>>();

        assert_eq!( output, vec![ &n(n(l(1), l(2)), l(3))
                                , &l(3), &n(l(1), l(2))
                                , &l(1)
                                ] );
    }
}
