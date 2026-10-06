//! Java 8 HashMap key iteration, including signed Long.compareTo in equal-hash tree bins.
const TREEIFY_THRESHOLD: usize = 8;
const UNTREEIFY_THRESHOLD: usize = 6;
const MIN_TREEIFY_CAPACITY: usize = 64;
const NIL: u32 = u32::MAX;

#[derive(Clone, Copy)]
struct Node<K> {
    hash: i32,
    key: K,
    next: u32,
    tree: bool,
    parent: u32,
    left: u32,
    right: u32,
    prev: u32,
    red: bool,
}

pub struct LongMapOrder<K> {
    nodes: Vec<Node<K>>,
    table: Vec<u32>,
    size: usize,
    threshold: usize,
}

impl<K: PartialEq + Copy + Ord> Default for LongMapOrder<K> {
    fn default() -> Self {
        LongMapOrder {
            nodes: Vec::new(),
            table: Vec::new(),
            size: 0,
            threshold: 0,
        }
    }
}

/// `HashMap.hash`
#[inline]
fn spread(h: i32) -> i32 {
    h ^ ((h as u32) >> 16) as i32
}

impl<K: PartialEq + Copy + Ord> LongMapOrder<K> {
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn first(&self) -> Option<K> {
        self.iter().next()
    }
    /// HashMap.remove(key), including movable tree-bin deletion from OpenJDK 8.
    pub fn remove(&mut self, hash_code: i32, key: K) -> bool {
        if self.table.is_empty() {
            return false;
        }
        let hash = spread(hash_code);
        let index = (self.table.len() - 1) & hash as usize;
        let mut e = self.table[index];
        let mut prev = NIL;
        while e != NIL && !(self.n(e).hash == hash && self.n(e).key == key) {
            prev = e;
            e = self.n(e).next;
        }
        if e == NIL {
            return false;
        }
        if self.n(e).tree {
            self.remove_tree_node(e, index);
        } else if prev == NIL {
            self.table[index] = self.n(e).next;
        } else {
            self.m(prev).next = self.n(e).next;
        }
        self.size -= 1;
        true
    }
    fn remove_tree_node(&mut self, p: u32, index: usize) {
        let mut first = self.table[index];
        let mut root = first;
        let succ = self.n(p).next;
        let pred = self.n(p).prev;
        if pred == NIL {
            first = succ;
            self.table[index] = first;
        } else {
            self.m(pred).next = succ;
        }
        if succ != NIL {
            self.m(succ).prev = pred;
        }
        if first == NIL {
            return;
        }
        while self.n(root).parent != NIL {
            root = self.n(root).parent;
        }
        let rl = self.n(root).left;
        if self.n(root).right == NIL || rl == NIL || self.n(rl).left == NIL {
            self.table[index] = self.untreeify(first);
            return;
        }
        let pl = self.n(p).left;
        let pr = self.n(p).right;
        let replacement;
        if pl != NIL && pr != NIL {
            let mut s = pr;
            while self.n(s).left != NIL {
                s = self.n(s).left;
            }
            let color = self.n(s).red;
            self.m(s).red = self.n(p).red;
            self.m(p).red = color;
            let sr = self.n(s).right;
            let pp = self.n(p).parent;
            if s == pr {
                self.m(p).parent = s;
                self.m(s).right = p;
            } else {
                let sp = self.n(s).parent;
                self.m(p).parent = sp;
                if sp != NIL {
                    if s == self.n(sp).left {
                        self.m(sp).left = p;
                    } else {
                        self.m(sp).right = p;
                    }
                }
                self.m(s).right = pr;
                self.m(pr).parent = s;
            }
            self.m(p).left = NIL;
            self.m(p).right = sr;
            if sr != NIL {
                self.m(sr).parent = p;
            }
            self.m(s).left = pl;
            self.m(pl).parent = s;
            self.m(s).parent = pp;
            if pp == NIL {
                root = s;
            } else if p == self.n(pp).left {
                self.m(pp).left = s;
            } else {
                self.m(pp).right = s;
            }
            replacement = if sr != NIL { sr } else { p };
        } else {
            replacement = if pl != NIL {
                pl
            } else if pr != NIL {
                pr
            } else {
                p
            };
        }
        if replacement != p {
            let pp = self.n(p).parent;
            self.m(replacement).parent = pp;
            if pp == NIL {
                root = replacement;
                self.m(root).red = false;
            } else if p == self.n(pp).left {
                self.m(pp).left = replacement;
            } else {
                self.m(pp).right = replacement;
            }
            self.m(p).left = NIL;
            self.m(p).right = NIL;
            self.m(p).parent = NIL;
        }
        let r = if self.n(p).red {
            root
        } else {
            self.balance_deletion(root, replacement)
        };
        if replacement == p {
            let pp = self.n(p).parent;
            self.m(p).parent = NIL;
            if pp != NIL {
                if p == self.n(pp).left {
                    self.m(pp).left = NIL;
                } else if p == self.n(pp).right {
                    self.m(pp).right = NIL;
                }
            }
        }
        self.move_root_to_front(r);
    }
    fn balance_deletion(&mut self, mut root: u32, mut x: u32) -> u32 {
        loop {
            if x == NIL || x == root {
                return root;
            }
            let mut xp = self.n(x).parent;
            if xp == NIL {
                self.m(x).red = false;
                return x;
            }
            if self.n(x).red {
                self.m(x).red = false;
                return root;
            }
            let left = self.n(xp).left == x;
            let mut sibling = if left {
                self.n(xp).right
            } else {
                self.n(xp).left
            };
            if sibling != NIL && self.n(sibling).red {
                self.m(sibling).red = false;
                self.m(xp).red = true;
                root = if left {
                    self.rotate_left(root, xp)
                } else {
                    self.rotate_right(root, xp)
                };
                xp = self.n(x).parent;
                sibling = if xp == NIL {
                    NIL
                } else if left {
                    self.n(xp).right
                } else {
                    self.n(xp).left
                };
            }
            if sibling == NIL {
                x = xp;
                continue;
            }
            let near = if left {
                self.n(sibling).left
            } else {
                self.n(sibling).right
            };
            let far = if left {
                self.n(sibling).right
            } else {
                self.n(sibling).left
            };
            if (near == NIL || !self.n(near).red) && (far == NIL || !self.n(far).red) {
                self.m(sibling).red = true;
                x = xp;
                continue;
            }
            if far == NIL || !self.n(far).red {
                if near != NIL {
                    self.m(near).red = false;
                }
                self.m(sibling).red = true;
                root = if left {
                    self.rotate_right(root, sibling)
                } else {
                    self.rotate_left(root, sibling)
                };
                xp = self.n(x).parent;
                sibling = if xp == NIL {
                    NIL
                } else if left {
                    self.n(xp).right
                } else {
                    self.n(xp).left
                };
            }
            if sibling != NIL {
                self.m(sibling).red = xp != NIL && self.n(xp).red;
                let far = if left {
                    self.n(sibling).right
                } else {
                    self.n(sibling).left
                };
                if far != NIL {
                    self.m(far).red = false;
                }
            }
            if xp != NIL {
                self.m(xp).red = false;
                root = if left {
                    self.rotate_left(root, xp)
                } else {
                    self.rotate_right(root, xp)
                };
            }
            x = root;
        }
    }
    #[inline]
    fn n(&self, i: u32) -> &Node<K> {
        &self.nodes[i as usize]
    }

    #[inline]
    fn m(&mut self, i: u32) -> &mut Node<K> {
        &mut self.nodes[i as usize]
    }

    fn new_node(&mut self, hash: i32, key: K, next: u32) -> u32 {
        self.nodes.push(Node {
            hash,
            key,
            next,
            tree: false,
            parent: NIL,
            left: NIL,
            right: NIL,
            prev: NIL,
            red: false,
        });
        (self.nodes.len() - 1) as u32
    }

    /// `HashSet.add`; returns false if the key was already present.
    pub fn insert(&mut self, hash_code: i32, key: K) -> bool {
        let hash = spread(hash_code);
        if self.table.is_empty() {
            self.resize();
        }
        let n = self.table.len();
        let i = (n - 1) & hash as usize;
        let p = self.table[i];
        if p == NIL {
            let x = self.new_node(hash, key, NIL);
            self.table[i] = x;
        } else if self.n(p).hash == hash && self.n(p).key == key {
            return false;
        } else if self.n(p).tree {
            if !self.put_tree_val(p, hash, key) {
                return false;
            }
        } else {
            let mut p = p;
            let mut bin_count = 0;
            loop {
                let e = self.n(p).next;
                if e == NIL {
                    let x = self.new_node(hash, key, NIL);
                    self.m(p).next = x;
                    if bin_count >= TREEIFY_THRESHOLD - 1 {
                        self.treeify_bin(hash);
                    }
                    break;
                }
                if self.n(e).hash == hash && self.n(e).key == key {
                    return false;
                }
                p = e;
                bin_count += 1;
            }
        }
        self.size += 1;
        if self.size > self.threshold {
            self.resize();
        }
        true
    }

    /// Iteration order of `HashSet.iterator()`.
    pub fn iter(&self) -> impl Iterator<Item = K> + '_ {
        self.table.iter().flat_map(move |&head| {
            let mut e = head;
            std::iter::from_fn(move || {
                if e == NIL {
                    return None;
                }
                let k = self.n(e).key;
                e = self.n(e).next;
                Some(k)
            })
        })
    }

    fn resize(&mut self) {
        let old_cap = self.table.len();
        let (new_cap, new_thr) = if old_cap > 0 {
            (old_cap << 1, self.threshold << 1)
        } else {
            (16, 12)
        };
        self.threshold = new_thr;
        let old = std::mem::replace(&mut self.table, vec![NIL; new_cap]);
        for (j, &e) in old.iter().enumerate() {
            if e == NIL {
                continue;
            }
            if self.n(e).next == NIL {
                let h = self.n(e).hash as usize & (new_cap - 1);
                self.table[h] = e;
            } else if self.n(e).tree {
                self.split(e, j, old_cap);
            } else {
                let (mut lo_h, mut lo_t, mut hi_h, mut hi_t) = (NIL, NIL, NIL, NIL);
                let mut e = e;
                while e != NIL {
                    let next = self.n(e).next;
                    if self.n(e).hash as usize & old_cap == 0 {
                        if lo_t == NIL {
                            lo_h = e;
                        } else {
                            self.m(lo_t).next = e;
                        }
                        lo_t = e;
                    } else {
                        if hi_t == NIL {
                            hi_h = e;
                        } else {
                            self.m(hi_t).next = e;
                        }
                        hi_t = e;
                    }
                    e = next;
                }
                if lo_t != NIL {
                    self.m(lo_t).next = NIL;
                    self.table[j] = lo_h;
                }
                if hi_t != NIL {
                    self.m(hi_t).next = NIL;
                    self.table[j + old_cap] = hi_h;
                }
            }
        }
    }

    fn treeify_bin(&mut self, hash: i32) {
        let n = self.table.len();
        if n < MIN_TREEIFY_CAPACITY {
            self.resize();
            return;
        }
        let index = (n - 1) & hash as usize;
        let mut e = self.table[index];
        let mut tl = NIL;
        while e != NIL {
            let next = self.n(e).next;
            let node = self.m(e);
            node.tree = true;
            node.prev = tl;
            node.parent = NIL;
            node.left = NIL;
            node.right = NIL;
            node.red = false;
            tl = e;
            e = next;
        }
        let hd = self.table[index];
        self.treeify(hd);
    }

    /// `TreeNode.treeify`
    fn treeify(&mut self, head: u32) {
        let mut root = NIL;
        let mut x = head;
        while x != NIL {
            let next = self.n(x).next;
            self.m(x).left = NIL;
            self.m(x).right = NIL;
            if root == NIL {
                self.m(x).parent = NIL;
                self.m(x).red = false;
                root = x;
            } else {
                let h = self.n(x).hash;
                let mut p = root;
                loop {
                    let ph = self.n(p).hash;
                    let left = ph > h || (ph == h && self.n(p).key > self.n(x).key);
                    let xp = p;
                    p = if left {
                        self.n(p).left
                    } else {
                        self.n(p).right
                    };
                    if p == NIL {
                        self.m(x).parent = xp;
                        if left {
                            self.m(xp).left = x;
                        } else {
                            self.m(xp).right = x;
                        }
                        root = self.balance_insertion(root, x);
                        break;
                    }
                }
            }
            x = next;
        }
        self.move_root_to_front(root);
    }

    /// `TreeNode.untreeify`: back to a plain list in the same order.
    fn untreeify(&mut self, head: u32) -> u32 {
        let mut q = head;
        while q != NIL {
            let node = self.m(q);
            node.tree = false;
            node.prev = NIL;
            node.parent = NIL;
            node.left = NIL;
            node.right = NIL;
            q = node.next;
        }
        head
    }

    /// `TreeNode.putTreeVal`; false if the key exists.
    fn put_tree_val(&mut self, first: u32, h: i32, key: K) -> bool {
        let mut root = first;
        while self.n(root).parent != NIL {
            root = self.n(root).parent;
        }
        let mut p = root;
        loop {
            let ph = self.n(p).hash;
            if ph == h && self.n(p).key == key {
                return false;
            }
            let left = ph > h || (ph == h && self.n(p).key > key);
            let xp = p;
            p = if left {
                self.n(p).left
            } else {
                self.n(p).right
            };
            if p == NIL {
                let xpn = self.n(xp).next;
                let x = self.new_node(h, key, xpn);
                self.m(x).tree = true;
                if left {
                    self.m(xp).left = x;
                } else {
                    self.m(xp).right = x;
                }
                self.m(xp).next = x;
                self.m(x).parent = xp;
                self.m(x).prev = xp;
                if xpn != NIL {
                    self.m(xpn).prev = x;
                }
                let r = self.balance_insertion(root, x);
                self.move_root_to_front(r);
                return true;
            }
        }
    }

    /// `TreeNode.split` during a resize.
    fn split(&mut self, b: u32, index: usize, bit: usize) {
        let (mut lo_h, mut lo_t, mut hi_h, mut hi_t) = (NIL, NIL, NIL, NIL);
        let (mut lc, mut hc) = (0, 0);
        let mut e = b;
        while e != NIL {
            let next = self.n(e).next;
            self.m(e).next = NIL;
            if self.n(e).hash as usize & bit == 0 {
                self.m(e).prev = lo_t;
                if lo_t == NIL {
                    lo_h = e;
                } else {
                    self.m(lo_t).next = e;
                }
                lo_t = e;
                lc += 1;
            } else {
                self.m(e).prev = hi_t;
                if hi_t == NIL {
                    hi_h = e;
                } else {
                    self.m(hi_t).next = e;
                }
                hi_t = e;
                hc += 1;
            }
            e = next;
        }
        if lo_h != NIL {
            if lc <= UNTREEIFY_THRESHOLD {
                self.table[index] = self.untreeify(lo_h);
            } else {
                self.table[index] = lo_h;
                if hi_h != NIL {
                    self.treeify(lo_h);
                }
            }
        }
        if hi_h != NIL {
            if hc <= UNTREEIFY_THRESHOLD {
                self.table[index + bit] = self.untreeify(hi_h);
            } else {
                self.table[index + bit] = hi_h;
                if lo_h != NIL {
                    self.treeify(hi_h);
                }
            }
        }
    }

    fn move_root_to_front(&mut self, root: u32) {
        let n = self.table.len();
        let index = (n - 1) & self.n(root).hash as usize;
        let first = self.table[index];
        if root != first {
            self.table[index] = root;
            let rp = self.n(root).prev;
            let rn = self.n(root).next;
            if rn != NIL {
                self.m(rn).prev = rp;
            }
            if rp != NIL {
                self.m(rp).next = rn;
            }
            if first != NIL {
                self.m(first).prev = root;
            }
            self.m(root).next = first;
            self.m(root).prev = NIL;
        }
    }

    fn rotate_left(&mut self, mut root: u32, p: u32) -> u32 {
        if p == NIL {
            return root;
        }
        let r = self.n(p).right;
        if r != NIL {
            let rl = self.n(r).left;
            self.m(p).right = rl;
            if rl != NIL {
                self.m(rl).parent = p;
            }
            let pp = self.n(p).parent;
            self.m(r).parent = pp;
            if pp == NIL {
                root = r;
                self.m(r).red = false;
            } else if self.n(pp).left == p {
                self.m(pp).left = r;
            } else {
                self.m(pp).right = r;
            }
            self.m(r).left = p;
            self.m(p).parent = r;
        }
        root
    }

    fn rotate_right(&mut self, mut root: u32, p: u32) -> u32 {
        if p == NIL {
            return root;
        }
        let l = self.n(p).left;
        if l != NIL {
            let lr = self.n(l).right;
            self.m(p).left = lr;
            if lr != NIL {
                self.m(lr).parent = p;
            }
            let pp = self.n(p).parent;
            self.m(l).parent = pp;
            if pp == NIL {
                root = l;
                self.m(l).red = false;
            } else if self.n(pp).right == p {
                self.m(pp).right = l;
            } else {
                self.m(pp).left = l;
            }
            self.m(l).right = p;
            self.m(p).parent = l;
        }
        root
    }

    fn balance_insertion(&mut self, mut root: u32, mut x: u32) -> u32 {
        self.m(x).red = true;
        loop {
            let mut xp = self.n(x).parent;
            if xp == NIL {
                self.m(x).red = false;
                return x;
            }
            if !self.n(xp).red {
                return root;
            }
            let mut xpp = self.n(xp).parent;
            if xpp == NIL {
                return root;
            }
            let xppl = self.n(xpp).left;
            if xp == xppl {
                let xppr = self.n(xpp).right;
                if xppr != NIL && self.n(xppr).red {
                    self.m(xppr).red = false;
                    self.m(xp).red = false;
                    self.m(xpp).red = true;
                    x = xpp;
                } else {
                    if x == self.n(xp).right {
                        x = xp;
                        root = self.rotate_left(root, x);
                        xp = self.n(x).parent;
                        xpp = if xp == NIL { NIL } else { self.n(xp).parent };
                    }
                    if xp != NIL {
                        self.m(xp).red = false;
                        if xpp != NIL {
                            self.m(xpp).red = true;
                            root = self.rotate_right(root, xpp);
                        }
                    }
                }
            } else if xppl != NIL && self.n(xppl).red {
                self.m(xppl).red = false;
                self.m(xp).red = false;
                self.m(xpp).red = true;
                x = xpp;
            } else {
                if x == self.n(xp).left {
                    x = xp;
                    root = self.rotate_right(root, x);
                    xp = self.n(x).parent;
                    xpp = if xp == NIL { NIL } else { self.n(xp).parent };
                }
                if xp != NIL {
                    self.m(xp).red = false;
                    if xpp != NIL {
                        self.m(xpp).red = true;
                        root = self.rotate_left(root, xpp);
                    }
                }
            }
        }
    }
}
