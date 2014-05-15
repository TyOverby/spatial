#![feature(struct_variant)]

use std::num::FromPrimitive;
use std::default::Default;

trait QTNumber: Num + Ord + FromPrimitive + Copy + Default {}

trait Point<N> {
    fn x(&self)-> N;
    fn y(&self)-> N;
}

struct Cardinal<T> {
    nw: T,
    ne: T,
    sw: T,
    se: T
}
impl <T> Cardinal<T> {
    fn new( nw: T, ne: T, sw: T, se: T) -> Cardinal<T> {
        Cardinal { nw: nw, ne: ne, sw: sw, se: se }
    }
}

#[deriving(Show, Eq, Copy)]
struct AABB<N> {
    x: N, y: N,
    w: N
}

impl <N: QTNumber> AABB<N> {
    fn new(x: N, y: N, w: N) -> AABB<N> {
        AABB {x: x, y: y, w: w}
    }

    fn split(&self) -> Cardinal<AABB<N>> {
        let two = FromPrimitive::from_uint(2).unwrap();
        let wot = self.w / two;
        let sw = AABB::new(self.x,       self.y,       wot);
        let nw = AABB::new(self.x,       self.y + wot, wot);
        let se = AABB::new(self.x + wot, self.y,       wot);
        let ne = AABB::new(self.x + wot, self.y + wot, wot);
        return Cardinal::new(nw, ne, sw, se);
    }
}
impl <N: QTNumber, P: Point<N>> AABB<N> {
    fn contains(&self, pt: &P) -> bool {
        let px = pt.x();
        let py = pt.y();

        if px < self.x { return false }
        if py < self.y { return false }
        if px > self.x + self.w { return false }
        if py > self.y + self.w { return false }

        return true;
    }
}

enum QuadTree<N, P> {
    Leaf {
        bounding: AABB<N>,
        contents: Vec<P>,
        cutoff: uint
    }, // One value
    Node {
        bounding: AABB<N>, // Split into quadrants
        children: Box<Cardinal<QuadTree<N, P>>>,
        cutoff: uint
    }
}
impl <N: QTNumber, P> QuadTree<N, P> {
    fn leaf_empty(bounding: AABB<N>, cutoff: uint) -> QuadTree<N, P>{
        Leaf {
            bounding: bounding,
            contents: Vec::with_capacity(cutoff),
            cutoff: cutoff
        }
    }
    fn leaf(bounding: AABB<N>, contents: Vec<P>, cutoff: uint) -> QuadTree<N, P> {
        Leaf {
            bounding: bounding,
            contents: contents,
            cutoff: cutoff
        }
    }
    fn node(bounding: AABB<N>, children: Box<Cardinal<QuadTree<N, P>>>, cutoff: uint) -> QuadTree<N, P>{
        Node {
            bounding: bounding,
            children: children,
            cutoff: cutoff
        }
    }
    pub fn new(bounding: AABB<N>, cutoff: uint) -> QuadTree<N, P> {
        let split = bounding.split();
        QuadTree::node(bounding, box Cardinal::new(
                QuadTree::leaf_empty(split.nw, cutoff),
                QuadTree::leaf_empty(split.ne, cutoff),
                QuadTree::leaf_empty(split.sw, cutoff),
                QuadTree::leaf_empty(split.se, cutoff)), cutoff)
    }
}

impl <N: QTNumber, P: Point<N> + Clone> QuadTree<N, P> {
    fn contains(&self, p: &P) -> bool {
        match self {
            &Leaf{ref bounding, ref contents, ref cutoff } => bounding.contains(p),
            &Node{ref bounding, ref children, ref cutoff } => bounding.contains(p),
        }
    }

    fn is_full(&self) -> bool {
        match self {
            &Node { ref bounding, ref children, ref cutoff } => false,
            &Leaf { ref bounding, ref contents, ref cutoff } => contents.len() == *cutoff
        }
    }

    fn breakup(&mut self) -> QuadTree<N, P> {
        match self {
            &Node {ref bounding, ref children, ref cutoff } => fail!("internal error: breakup() on Node"),
            &Leaf {ref bounding, ref mut contents, cutoff } => {
                let bb = bounding.split();
                let mut node = QuadTree::node(*bounding, box Cardinal::new(
                        QuadTree::leaf_empty(bb.nw, cutoff), QuadTree::leaf_empty(bb.ne, cutoff),
                        QuadTree::leaf_empty(bb.sw, cutoff), QuadTree::leaf_empty(bb.se, cutoff)), cutoff);

                for point in contents.iter() {
                    node.insert(point.clone());
                }

                node
            }
        }
    }

    fn insert(&mut self, p: P) -> bool {
        if !self.contains(&p) {
            return false;
        }
        match self {
            &Leaf{ ref bounding, ref mut contents, cutoff } => {
                contents.push(p);
            }
            &Node{ ref bounding, ref mut children, cutoff } => {
                let mut rep = &mut children.se;
                if children.nw.contains(&p) {
                    rep = &mut children.nw;
                } else if children.ne.contains(&p) {
                    rep = &mut children.ne;
                } else if children.sw.contains(&p) {
                    rep = &mut children.sw;
                }
                assert!(rep.contains(&p), "{:?}, {:?}", rep, p);
                if !rep.is_full() {
                    rep.insert(p);
                } else {
                    *rep = rep.breakup();
                    rep.insert(p);
                }
            }
        };
        return true;
    }
}

fn main() {}

#[test]
fn testBasic() {
    #[deriving(Show)]
    impl QTNumber for f64 {}
    #[deriving(Clone)]
    struct Pt { x:f64, y: f64}
    impl Point<f64> for Pt {
        fn x(&self)->f64{ self.x }
        fn y(&self)->f64{ self.y }
    }
    impl Pt {
        fn new(x: f64, y: f64) -> Pt {
            Pt { x: x, y: y }
        }
    }

    let mut tree:QuadTree<f64, Pt> = QuadTree::new(AABB::new(0.0, 0.0, 100.0), 4);

    tree.insert(Pt::new(0.0, 0.0));
    tree.insert(Pt::new(1.0, 1.0));
    tree.insert(Pt::new(2.0, 2.0));
    tree.insert(Pt::new(3.0, 3.0));
    tree.insert(Pt::new(4.0, 4.0));
    tree.insert(Pt::new(5.0, 5.0));

    println!("{:?}", tree);
    assert!(false);
}
