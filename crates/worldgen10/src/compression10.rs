//! Java's `java.util.zip.Deflater` at its default level, which runs zlib's `deflate`.
//! Release 1.0 writes level.dat/players with `GZIPOutputStream` (raw deflate inside
//! Java's own fixed GZIP header) and region chunks with `DeflaterOutputStream` (zlib
//! wrapper). miniz and zlib-ng choose different matches and blocks, so this is a
//! line-by-line port of zlib's level-6 path: deflate.c (`fill_window`, `longest_match`,
//! `deflate_slow`) and trees.c (Huffman tree construction and block emission), with
//! windowBits 15 and memLevel 8 as `Deflater` uses. Java feeds the stream in pieces
//! with `Z_NO_FLUSH` and then finishes; zlib's output does not depend on that input
//! chunking (matching only runs with at least `MIN_LOOKAHEAD` bytes buffered), so the
//! whole input is compressed in one `Z_FINISH` pass here.
use std::{io, sync::OnceLock};

const MIN_MATCH: usize = 3;
const MAX_MATCH: usize = 258;
const W_SIZE: usize = 1 << 15;
const W_MASK: usize = W_SIZE - 1;
const WINDOW_SIZE: usize = 2 * W_SIZE;
const HASH_SIZE: usize = 1 << 15;
const HASH_MASK: usize = HASH_SIZE - 1;
const HASH_SHIFT: usize = 5;
const MIN_LOOKAHEAD: usize = MAX_MATCH + MIN_MATCH + 1;
const MAX_DIST: usize = W_SIZE - MIN_LOOKAHEAD;
const WIN_INIT: usize = MAX_MATCH;
const LIT_BUFSIZE: usize = 1 << 14;
const TOO_FAR: u32 = 4096;
// configuration_table[6]
const GOOD_MATCH: usize = 8;
const MAX_LAZY: usize = 16;
const NICE_MATCH: usize = 128;
const MAX_CHAIN: usize = 128;

const LENGTH_CODES: usize = 29;
const LITERALS: usize = 256;
const L_CODES: usize = LITERALS + 1 + LENGTH_CODES;
const D_CODES: usize = 30;
const BL_CODES: usize = 19;
const HEAP_SIZE: usize = 2 * L_CODES + 1;
const MAX_BITS: usize = 15;
const MAX_BL_BITS: i32 = 7;
const END_BLOCK: usize = 256;
const REP_3_6: usize = 16;
const REPZ_3_10: usize = 17;
const REPZ_11_138: usize = 18;
const STORED_BLOCK: u32 = 0;
const STATIC_TREES: u32 = 1;
const DYN_TREES: u32 = 2;

const EXTRA_LBITS: [i32; LENGTH_CODES] =
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const EXTRA_DBITS: [i32; D_CODES] =
    [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const EXTRA_BLBITS: [i32; BL_CODES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7];
const BL_ORDER: [usize; BL_CODES] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

/// trees.c `tr_static_init` results.
struct Static {
    length_code: [u8; MAX_MATCH - MIN_MATCH + 1],
    dist_code: [u8; 512],
    base_length: [i32; LENGTH_CODES],
    base_dist: [i32; D_CODES],
    ltree_code: [u16; L_CODES + 2],
    ltree_len: [u16; L_CODES + 2],
    dtree_code: [u16; D_CODES],
    dtree_len: [u16; D_CODES],
}

fn bi_reverse(mut code: u32, mut len: i32) -> u32 {
    let mut res = 0;
    loop {
        res |= code & 1;
        code >>= 1;
        res <<= 1;
        len -= 1;
        if len <= 0 {
            break;
        }
    }
    res >> 1
}

/// trees.c `gen_codes`.
fn gen_codes(code: &mut [u16], len: &[u16], max_code: usize, bl_count: &[u16; MAX_BITS + 1]) {
    let mut next_code = [0u16; MAX_BITS + 1];
    let mut c: u32 = 0;
    for bits in 1..=MAX_BITS {
        c = (c + bl_count[bits - 1] as u32) << 1;
        next_code[bits] = c as u16;
    }
    for n in 0..=max_code {
        let l = len[n] as usize;
        if l == 0 {
            continue;
        }
        code[n] = bi_reverse(next_code[l] as u32, l as i32) as u16;
        next_code[l] = next_code[l].wrapping_add(1);
    }
}

fn tables() -> &'static Static {
    static TABLES: OnceLock<Static> = OnceLock::new();
    TABLES.get_or_init(|| {
        let mut s = Static {
            length_code: [0; MAX_MATCH - MIN_MATCH + 1],
            dist_code: [0; 512],
            base_length: [0; LENGTH_CODES],
            base_dist: [0; D_CODES],
            ltree_code: [0; L_CODES + 2],
            ltree_len: [0; L_CODES + 2],
            dtree_code: [0; D_CODES],
            dtree_len: [0; D_CODES],
        };
        let mut length = 0usize;
        let mut code = 0usize;
        while code < LENGTH_CODES - 1 {
            s.base_length[code] = length as i32;
            for _ in 0..1 << EXTRA_LBITS[code] {
                s.length_code[length] = code as u8;
                length += 1;
            }
            code += 1;
        }
        s.length_code[length - 1] = code as u8;
        let mut dist = 0usize;
        code = 0;
        while code < 16 {
            s.base_dist[code] = dist as i32;
            for _ in 0..1 << EXTRA_DBITS[code] {
                s.dist_code[dist] = code as u8;
                dist += 1;
            }
            code += 1;
        }
        dist >>= 7;
        while code < D_CODES {
            s.base_dist[code] = (dist << 7) as i32;
            for _ in 0..1 << (EXTRA_DBITS[code] - 7) {
                s.dist_code[256 + dist] = code as u8;
                dist += 1;
            }
            code += 1;
        }
        let mut bl_count = [0u16; MAX_BITS + 1];
        for n in 0..L_CODES + 2 {
            let l = match n {
                0..=143 => 8,
                144..=255 => 9,
                256..=279 => 7,
                _ => 8,
            };
            s.ltree_len[n] = l;
            bl_count[l as usize] += 1;
        }
        let len = s.ltree_len;
        gen_codes(&mut s.ltree_code, &len, L_CODES + 1, &bl_count);
        for n in 0..D_CODES {
            s.dtree_len[n] = 5;
            s.dtree_code[n] = bi_reverse(n as u32, 5) as u16;
        }
        s
    })
}

#[inline]
fn d_code(t: &Static, dist: usize) -> usize {
    if dist < 256 {
        t.dist_code[dist] as usize
    } else {
        t.dist_code[256 + (dist >> 7)] as usize
    }
}

/// A `ct_data` array; zlib's Freq/Code and Dad/Len unions are kept apart, which is
/// equivalent because every union member is written before it is read.
struct Tree {
    freq: Vec<u16>,
    code: Vec<u16>,
    dad: Vec<u16>,
    len: Vec<u16>,
    max_code: i32,
}
impl Tree {
    fn new(size: usize) -> Self {
        Tree { freq: vec![0; size], code: vec![0; size], dad: vec![0; size], len: vec![0; size], max_code: 0 }
    }
}

/// `static_tree_desc`
struct Desc {
    stree_len: Option<&'static [u16]>,
    extra: &'static [i32],
    extra_base: usize,
    elems: usize,
    max_length: i32,
}

/// The tree-building scratch state of `deflate_state`.
struct Builder {
    bl_count: [u16; MAX_BITS + 1],
    heap: [i32; HEAP_SIZE],
    heap_len: usize,
    heap_max: usize,
    depth: [u8; HEAP_SIZE],
    opt_len: u64,
    static_len: u64,
}

#[inline]
fn smaller(tree: &Tree, n: usize, m: usize, depth: &[u8]) -> bool {
    tree.freq[n] < tree.freq[m] || (tree.freq[n] == tree.freq[m] && depth[n] <= depth[m])
}

impl Builder {
    fn pqdownheap(&mut self, tree: &Tree, mut k: usize) {
        let v = self.heap[k];
        let mut j = k << 1;
        while j <= self.heap_len {
            if j < self.heap_len && smaller(tree, self.heap[j + 1] as usize, self.heap[j] as usize, &self.depth) {
                j += 1;
            }
            if smaller(tree, v as usize, self.heap[j] as usize, &self.depth) {
                break;
            }
            self.heap[k] = self.heap[j];
            k = j;
            j <<= 1;
        }
        self.heap[k] = v;
    }

    fn gen_bitlen(&mut self, tree: &mut Tree, desc: &Desc) {
        let max_code = tree.max_code;
        let mut overflow = 0;
        self.bl_count = [0; MAX_BITS + 1];
        tree.len[self.heap[self.heap_max] as usize] = 0;
        let mut h = self.heap_max + 1;
        while h < HEAP_SIZE {
            let n = self.heap[h] as usize;
            let mut bits = tree.len[tree.dad[n] as usize] as i32 + 1;
            if bits > desc.max_length {
                bits = desc.max_length;
                overflow += 1;
            }
            tree.len[n] = bits as u16;
            h += 1;
            if n as i32 > max_code {
                continue;
            }
            self.bl_count[bits as usize] += 1;
            let mut xbits = 0;
            if n >= desc.extra_base {
                xbits = desc.extra[n - desc.extra_base];
            }
            let f = tree.freq[n] as u64;
            self.opt_len = self.opt_len.wrapping_add(f * (bits + xbits) as u64);
            if let Some(stree) = desc.stree_len {
                self.static_len = self.static_len.wrapping_add(f * (stree[n] as i32 + xbits) as u64);
            }
        }
        if overflow == 0 {
            return;
        }
        loop {
            let mut bits = (desc.max_length - 1) as usize;
            while self.bl_count[bits] == 0 {
                bits -= 1;
            }
            self.bl_count[bits] -= 1;
            self.bl_count[bits + 1] += 2;
            self.bl_count[desc.max_length as usize] -= 1;
            overflow -= 2;
            if overflow <= 0 {
                break;
            }
        }
        let mut h = HEAP_SIZE;
        let mut bits = desc.max_length as usize;
        while bits != 0 {
            let mut n = self.bl_count[bits];
            while n != 0 {
                h -= 1;
                let m = self.heap[h] as usize;
                if m as i32 > max_code {
                    continue;
                }
                if tree.len[m] as usize != bits {
                    self.opt_len = self
                        .opt_len
                        .wrapping_add((bits as u64).wrapping_sub(tree.len[m] as u64).wrapping_mul(tree.freq[m] as u64));
                    tree.len[m] = bits as u16;
                }
                n -= 1;
            }
            bits -= 1;
        }
    }

    fn build_tree(&mut self, tree: &mut Tree, desc: &Desc) {
        let elems = desc.elems;
        let mut max_code: i32 = -1;
        self.heap_len = 0;
        self.heap_max = HEAP_SIZE;
        for n in 0..elems {
            if tree.freq[n] != 0 {
                self.heap_len += 1;
                self.heap[self.heap_len] = n as i32;
                max_code = n as i32;
                self.depth[n] = 0;
            } else {
                tree.len[n] = 0;
            }
        }
        while self.heap_len < 2 {
            let node = if max_code < 2 {
                max_code += 1;
                max_code
            } else {
                0
            } as usize;
            self.heap_len += 1;
            self.heap[self.heap_len] = node as i32;
            tree.freq[node] = 1;
            self.depth[node] = 0;
            self.opt_len = self.opt_len.wrapping_sub(1);
            if let Some(stree) = desc.stree_len {
                self.static_len = self.static_len.wrapping_sub(stree[node] as u64);
            }
        }
        tree.max_code = max_code;
        let mut n = self.heap_len / 2;
        while n >= 1 {
            self.pqdownheap(tree, n);
            n -= 1;
        }
        let mut node = elems;
        loop {
            // pqremove
            let n = self.heap[1] as usize;
            self.heap[1] = self.heap[self.heap_len];
            self.heap_len -= 1;
            self.pqdownheap(tree, 1);
            let m = self.heap[1] as usize;
            self.heap_max -= 1;
            self.heap[self.heap_max] = n as i32;
            self.heap_max -= 1;
            self.heap[self.heap_max] = m as i32;
            tree.freq[node] = tree.freq[n].wrapping_add(tree.freq[m]);
            self.depth[node] = (if self.depth[n] >= self.depth[m] { self.depth[n] } else { self.depth[m] }) + 1;
            tree.dad[n] = node as u16;
            tree.dad[m] = node as u16;
            self.heap[1] = node as i32;
            node += 1;
            self.pqdownheap(tree, 1);
            if self.heap_len < 2 {
                break;
            }
        }
        self.heap_max -= 1;
        self.heap[self.heap_max] = self.heap[1];
        self.gen_bitlen(tree, desc);
        let bl_count = self.bl_count;
        let len = tree.len.clone();
        gen_codes(&mut tree.code, &len, max_code.max(-1) as usize, &bl_count);
    }
}

/// trees.c `scan_tree`: tally the run-length codes of one tree into `bl_tree`.
fn scan_tree(tree: &mut Tree, bl_tree: &mut Tree, max_code: i32) {
    let mut prevlen: i32 = -1;
    let mut nextlen = tree.len[0] as i32;
    let mut count = 0;
    let (mut max_count, mut min_count) = (7, 4);
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    tree.len[(max_code + 1) as usize] = 0xffff;
    for n in 0..=max_code as usize {
        let curlen = nextlen;
        nextlen = tree.len[n + 1] as i32;
        count += 1;
        if count < max_count && curlen == nextlen {
            continue;
        } else if count < min_count {
            bl_tree.freq[curlen as usize] = bl_tree.freq[curlen as usize].wrapping_add(count as u16);
        } else if curlen != 0 {
            if curlen != prevlen {
                bl_tree.freq[curlen as usize] += 1;
            }
            bl_tree.freq[REP_3_6] += 1;
        } else if count <= 10 {
            bl_tree.freq[REPZ_3_10] += 1;
        } else {
            bl_tree.freq[REPZ_11_138] += 1;
        }
        count = 0;
        prevlen = curlen;
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        } else if curlen == nextlen {
            max_count = 6;
            min_count = 3;
        } else {
            max_count = 7;
            min_count = 4;
        }
    }
}

struct Deflate<'a> {
    t: &'static Static,
    input: &'a [u8],
    next_in: usize,
    window: Vec<u8>,
    prev: Vec<u16>,
    head: Vec<u16>,
    ins_h: usize,
    strstart: usize,
    block_start: i64,
    lookahead: usize,
    insert: usize,
    match_length: usize,
    prev_length: usize,
    match_available: bool,
    match_start: u32,
    prev_match: u32,
    high_water: usize,
    ltree: Tree,
    dtree: Tree,
    bl_tree: Tree,
    builder: Builder,
    syms: Vec<(u16, u8)>,
    bi_buf: u64,
    bi_valid: u32,
    out: Vec<u8>,
}

impl<'a> Deflate<'a> {
    fn new(input: &'a [u8]) -> Self {
        let mut d = Deflate {
            t: tables(),
            input,
            next_in: 0,
            window: vec![0; WINDOW_SIZE],
            prev: vec![0; W_SIZE],
            head: vec![0; HASH_SIZE],
            ins_h: 0,
            strstart: 0,
            block_start: 0,
            lookahead: 0,
            insert: 0,
            match_length: MIN_MATCH - 1,
            prev_length: MIN_MATCH - 1,
            match_available: false,
            match_start: 0,
            prev_match: 0,
            high_water: 0,
            ltree: Tree::new(HEAP_SIZE),
            dtree: Tree::new(2 * D_CODES + 1),
            bl_tree: Tree::new(2 * BL_CODES + 1),
            builder: Builder {
                bl_count: [0; MAX_BITS + 1],
                heap: [0; HEAP_SIZE],
                heap_len: 0,
                heap_max: 0,
                depth: [0; HEAP_SIZE],
                opt_len: 0,
                static_len: 0,
            },
            syms: Vec::with_capacity(LIT_BUFSIZE),
            bi_buf: 0,
            bi_valid: 0,
            out: Vec::with_capacity(input.len() / 2 + 64),
        };
        d.init_block();
        d
    }

    // ------------------------------------------------------------------ bit output
    #[inline]
    fn send_bits(&mut self, value: u32, length: u32) {
        self.bi_buf |= (value as u64) << self.bi_valid;
        self.bi_valid += length;
        while self.bi_valid >= 16 {
            self.out.push(self.bi_buf as u8);
            self.out.push((self.bi_buf >> 8) as u8);
            self.bi_buf >>= 16;
            self.bi_valid -= 16;
        }
    }
    /// `bi_windup`: flush the remaining bits, padding to a byte boundary.
    fn bi_windup(&mut self) {
        while self.bi_valid > 0 {
            self.out.push(self.bi_buf as u8);
            self.bi_buf >>= 8;
            self.bi_valid = self.bi_valid.saturating_sub(8);
        }
        self.bi_buf = 0;
    }

    // ------------------------------------------------------------------ trees.c
    fn init_block(&mut self) {
        for n in 0..L_CODES {
            self.ltree.freq[n] = 0;
        }
        for n in 0..D_CODES {
            self.dtree.freq[n] = 0;
        }
        for n in 0..BL_CODES {
            self.bl_tree.freq[n] = 0;
        }
        self.ltree.freq[END_BLOCK] = 1;
        self.builder.opt_len = 0;
        self.builder.static_len = 0;
        self.syms.clear();
    }

    fn tally_lit(&mut self, c: u8) -> bool {
        self.syms.push((0, c));
        self.ltree.freq[c as usize] += 1;
        self.syms.len() == LIT_BUFSIZE - 1
    }

    fn tally_dist(&mut self, dist: usize, len: usize) -> bool {
        self.syms.push((dist as u16, len as u8));
        let d = (dist as u16).wrapping_sub(1) as usize;
        self.ltree.freq[self.t.length_code[len] as usize + LITERALS + 1] += 1;
        self.dtree.freq[d_code(self.t, d)] += 1;
        self.syms.len() == LIT_BUFSIZE - 1
    }

    fn send_tree(&mut self, which: usize, max_code: i32) {
        let mut prevlen: i32 = -1;
        let tree_len = |s: &Self, n: usize| if which == 0 { s.ltree.len[n] } else { s.dtree.len[n] } as i32;
        let mut nextlen = tree_len(self, 0);
        let mut count = 0;
        let (mut max_count, mut min_count) = (7, 4);
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        }
        for n in 0..=max_code as usize {
            let curlen = nextlen;
            nextlen = tree_len(self, n + 1);
            count += 1;
            if count < max_count && curlen == nextlen {
                continue;
            } else if count < min_count {
                loop {
                    self.send_bl_code(curlen as usize);
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
            } else if curlen != 0 {
                if curlen != prevlen {
                    self.send_bl_code(curlen as usize);
                    count -= 1;
                }
                self.send_bl_code(REP_3_6);
                self.send_bits((count - 3) as u32, 2);
            } else if count <= 10 {
                self.send_bl_code(REPZ_3_10);
                self.send_bits((count - 3) as u32, 3);
            } else {
                self.send_bl_code(REPZ_11_138);
                self.send_bits((count - 11) as u32, 7);
            }
            count = 0;
            prevlen = curlen;
            if nextlen == 0 {
                max_count = 138;
                min_count = 3;
            } else if curlen == nextlen {
                max_count = 6;
                min_count = 3;
            } else {
                max_count = 7;
                min_count = 4;
            }
        }
    }

    #[inline]
    fn send_bl_code(&mut self, c: usize) {
        self.send_bits(self.bl_tree.code[c] as u32, self.bl_tree.len[c] as u32);
    }

    fn build_bl_tree(&mut self) -> usize {
        let lmax = self.ltree.max_code;
        let dmax = self.dtree.max_code;
        scan_tree(&mut self.ltree, &mut self.bl_tree, lmax);
        scan_tree(&mut self.dtree, &mut self.bl_tree, dmax);
        let desc = Desc { stree_len: None, extra: &EXTRA_BLBITS, extra_base: 0, elems: BL_CODES, max_length: MAX_BL_BITS };
        self.builder.build_tree(&mut self.bl_tree, &desc);
        let mut max_blindex = BL_CODES - 1;
        while max_blindex >= 3 {
            if self.bl_tree.len[BL_ORDER[max_blindex]] != 0 {
                break;
            }
            max_blindex -= 1;
        }
        self.builder.opt_len = self.builder.opt_len.wrapping_add(3 * (max_blindex as u64 + 1) + 5 + 5 + 4);
        max_blindex
    }

    fn send_all_trees(&mut self, lcodes: usize, dcodes: usize, blcodes: usize) {
        self.send_bits((lcodes - 257) as u32, 5);
        self.send_bits((dcodes - 1) as u32, 5);
        self.send_bits((blcodes - 4) as u32, 4);
        for rank in 0..blcodes {
            self.send_bits(self.bl_tree.len[BL_ORDER[rank]] as u32, 3);
        }
        self.send_tree(0, lcodes as i32 - 1);
        self.send_tree(1, dcodes as i32 - 1);
    }

    /// `compress_block` with the static (`dynamic == false`) or dynamic trees.
    fn compress_block(&mut self, dynamic: bool) {
        let t = self.t;
        let syms = std::mem::take(&mut self.syms);
        let lcode = |s: &Self, c: usize| -> (u32, u32) {
            if dynamic {
                (s.ltree.code[c] as u32, s.ltree.len[c] as u32)
            } else {
                (t.ltree_code[c] as u32, t.ltree_len[c] as u32)
            }
        };
        let dcode = |s: &Self, c: usize| -> (u32, u32) {
            if dynamic {
                (s.dtree.code[c] as u32, s.dtree.len[c] as u32)
            } else {
                (t.dtree_code[c] as u32, t.dtree_len[c] as u32)
            }
        };
        for &(dist, lc) in &syms {
            let mut lc = lc as i32;
            if dist == 0 {
                let (c, l) = lcode(self, lc as usize);
                self.send_bits(c, l);
            } else {
                let code = t.length_code[lc as usize] as usize;
                let (c, l) = lcode(self, code + LITERALS + 1);
                self.send_bits(c, l);
                let extra = EXTRA_LBITS[code];
                if extra != 0 {
                    lc -= t.base_length[code];
                    self.send_bits(lc as u32, extra as u32);
                }
                let mut dist = dist as u32 - 1;
                let code = d_code(t, dist as usize);
                let (c, l) = dcode(self, code);
                self.send_bits(c, l);
                let extra = EXTRA_DBITS[code];
                if extra != 0 {
                    dist -= t.base_dist[code] as u32;
                    self.send_bits(dist, extra as u32);
                }
            }
        }
        let (c, l) = lcode(self, END_BLOCK);
        self.send_bits(c, l);
        self.syms = syms;
    }

    fn stored_block(&mut self, buf: usize, stored_len: usize, last: bool) {
        self.send_bits((STORED_BLOCK << 1) + last as u32, 3);
        self.bi_windup();
        let len = stored_len as u16;
        self.out.extend(len.to_le_bytes());
        self.out.extend((!len).to_le_bytes());
        self.out.extend_from_slice(&self.window[buf..buf + stored_len]);
    }

    /// `_tr_flush_block`
    fn tr_flush_block(&mut self, buf: Option<usize>, stored_len: usize, last: bool) {
        let t = self.t;
        let ldesc = Desc {
            stree_len: Some(&t.ltree_len),
            extra: &EXTRA_LBITS,
            extra_base: LITERALS + 1,
            elems: L_CODES,
            max_length: MAX_BITS as i32,
        };
        let ddesc =
            Desc { stree_len: Some(&t.dtree_len), extra: &EXTRA_DBITS, extra_base: 0, elems: D_CODES, max_length: MAX_BITS as i32 };
        self.builder.build_tree(&mut self.ltree, &ldesc);
        self.builder.build_tree(&mut self.dtree, &ddesc);
        let max_blindex = self.build_bl_tree();
        let mut opt_lenb = self.builder.opt_len.wrapping_add(3 + 7) >> 3;
        let static_lenb = self.builder.static_len.wrapping_add(3 + 7) >> 3;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
        match buf {
            Some(buf) if stored_len as u64 + 4 <= opt_lenb => self.stored_block(buf, stored_len, last),
            _ if static_lenb == opt_lenb => {
                self.send_bits((STATIC_TREES << 1) + last as u32, 3);
                self.compress_block(false);
            }
            _ => {
                self.send_bits((DYN_TREES << 1) + last as u32, 3);
                let (l, d) = ((self.ltree.max_code + 1) as usize, (self.dtree.max_code + 1) as usize);
                self.send_all_trees(l, d, max_blindex + 1);
                self.compress_block(true);
            }
        }
        self.init_block();
        if last {
            self.bi_windup();
        }
    }

    // ------------------------------------------------------------------ deflate.c
    fn flush_block(&mut self, last: bool) {
        let buf = (self.block_start >= 0).then_some(self.block_start as usize);
        let stored_len = (self.strstart as i64 - self.block_start) as usize;
        self.tr_flush_block(buf, stored_len, last);
        self.block_start = self.strstart as i64;
    }

    #[inline]
    fn insert_string(&mut self, s: usize) -> usize {
        self.ins_h = ((self.ins_h << HASH_SHIFT) ^ self.window[s + MIN_MATCH - 1] as usize) & HASH_MASK;
        let head = self.head[self.ins_h];
        self.prev[s & W_MASK] = head;
        self.head[self.ins_h] = s as u16;
        head as usize
    }

    fn slide_hash(&mut self) {
        for p in self.head.iter_mut().chain(self.prev.iter_mut()) {
            let m = *p as usize;
            *p = if m >= W_SIZE { (m - W_SIZE) as u16 } else { 0 };
        }
    }

    fn fill_window(&mut self) {
        loop {
            let mut more = WINDOW_SIZE - self.lookahead - self.strstart;
            if self.strstart >= W_SIZE + MAX_DIST {
                self.window.copy_within(W_SIZE..W_SIZE + (W_SIZE - more), 0);
                self.match_start = self.match_start.wrapping_sub(W_SIZE as u32);
                self.strstart -= W_SIZE;
                self.block_start -= W_SIZE as i64;
                if self.insert > self.strstart {
                    self.insert = self.strstart;
                }
                self.slide_hash();
                more += W_SIZE;
            }
            let avail = self.input.len() - self.next_in;
            if avail == 0 {
                break;
            }
            let n = avail.min(more);
            let at = self.strstart + self.lookahead;
            self.window[at..at + n].copy_from_slice(&self.input[self.next_in..self.next_in + n]);
            self.next_in += n;
            self.lookahead += n;
            if self.lookahead + self.insert >= MIN_MATCH {
                let mut s = self.strstart - self.insert;
                self.ins_h = self.window[s] as usize;
                self.ins_h = ((self.ins_h << HASH_SHIFT) ^ self.window[s + 1] as usize) & HASH_MASK;
                while self.insert != 0 {
                    self.ins_h = ((self.ins_h << HASH_SHIFT) ^ self.window[s + MIN_MATCH - 1] as usize) & HASH_MASK;
                    self.prev[s & W_MASK] = self.head[self.ins_h];
                    self.head[self.ins_h] = s as u16;
                    s += 1;
                    self.insert -= 1;
                    if self.lookahead + self.insert < MIN_MATCH {
                        break;
                    }
                }
            }
            if !(self.lookahead < MIN_LOOKAHEAD && self.next_in < self.input.len()) {
                break;
            }
        }
        if self.high_water < WINDOW_SIZE {
            let curr = self.strstart + self.lookahead;
            if self.high_water < curr {
                let init = (WINDOW_SIZE - curr).min(WIN_INIT);
                self.window[curr..curr + init].fill(0);
                self.high_water = curr + init;
            } else if self.high_water < curr + WIN_INIT {
                let init = (curr + WIN_INIT - self.high_water).min(WINDOW_SIZE - self.high_water);
                self.window[self.high_water..self.high_water + init].fill(0);
                self.high_water += init;
            }
        }
    }

    fn longest_match(&mut self, mut cur_match: usize) -> usize {
        let mut chain_length = MAX_CHAIN;
        let scan = self.strstart;
        let mut best_len = self.prev_length;
        let mut nice_match = NICE_MATCH;
        let limit = if self.strstart > MAX_DIST { self.strstart - MAX_DIST } else { 0 };
        let strend = scan + MAX_MATCH;
        let w = &self.window;
        let mut scan_end1 = w[scan + best_len - 1];
        let mut scan_end = w[scan + best_len];
        if self.prev_length >= GOOD_MATCH {
            chain_length >>= 2;
        }
        if nice_match > self.lookahead {
            nice_match = self.lookahead;
        }
        loop {
            let m = cur_match;
            if !(w[m + best_len] != scan_end || w[m + best_len - 1] != scan_end1 || w[m] != w[scan] || w[m + 1] != w[scan + 1]) {
                // scan += 2, match++: scan[2] and match[2] are equal by the hash.
                let mut s = scan + 2;
                let mut mm = m + 2;
                'compare: loop {
                    for _ in 0..8 {
                        s += 1;
                        mm += 1;
                        if w[s] != w[mm] {
                            break 'compare;
                        }
                    }
                    if s >= strend {
                        break;
                    }
                }
                let len = MAX_MATCH - (strend - s);
                if len > best_len {
                    self.match_start = cur_match as u32;
                    best_len = len;
                    if len >= nice_match {
                        break;
                    }
                    scan_end1 = w[scan + best_len - 1];
                    scan_end = w[scan + best_len];
                }
            }
            cur_match = self.prev[cur_match & W_MASK] as usize;
            if cur_match <= limit {
                break;
            }
            chain_length -= 1;
            if chain_length == 0 {
                break;
            }
        }
        if best_len <= self.lookahead {
            best_len
        } else {
            self.lookahead
        }
    }

    /// `deflate_slow(s, Z_FINISH)` over the complete input.
    fn deflate_slow(&mut self) {
        loop {
            if self.lookahead < MIN_LOOKAHEAD {
                self.fill_window();
                if self.lookahead == 0 {
                    break;
                }
            }
            let mut hash_head = 0;
            if self.lookahead >= MIN_MATCH {
                hash_head = self.insert_string(self.strstart);
            }
            self.prev_length = self.match_length;
            self.prev_match = self.match_start;
            self.match_length = MIN_MATCH - 1;
            if hash_head != 0 && self.prev_length < MAX_LAZY && self.strstart - hash_head <= MAX_DIST {
                self.match_length = self.longest_match(hash_head);
                if self.match_length <= 5
                    && self.match_length == MIN_MATCH
                    && (self.strstart as u32).wrapping_sub(self.match_start) > TOO_FAR
                {
                    self.match_length = MIN_MATCH - 1;
                }
            }
            if self.prev_length >= MIN_MATCH && self.match_length <= self.prev_length {
                let max_insert = self.strstart + self.lookahead - MIN_MATCH;
                let dist = (self.strstart as u32 - 1).wrapping_sub(self.prev_match) as usize;
                let bflush = self.tally_dist(dist, self.prev_length - MIN_MATCH);
                self.lookahead -= self.prev_length - 1;
                self.prev_length -= 2;
                loop {
                    self.strstart += 1;
                    if self.strstart <= max_insert {
                        self.insert_string(self.strstart);
                    }
                    self.prev_length -= 1;
                    if self.prev_length == 0 {
                        break;
                    }
                }
                self.match_available = false;
                self.match_length = MIN_MATCH - 1;
                self.strstart += 1;
                if bflush {
                    self.flush_block(false);
                }
            } else if self.match_available {
                let bflush = self.tally_lit(self.window[self.strstart - 1]);
                if bflush {
                    self.flush_block(false);
                }
                self.strstart += 1;
                self.lookahead -= 1;
            } else {
                self.match_available = true;
                self.strstart += 1;
                self.lookahead -= 1;
            }
        }
        if self.match_available {
            self.tally_lit(self.window[self.strstart - 1]);
            self.match_available = false;
        }
        self.insert = self.strstart.min(MIN_MATCH - 1);
        self.flush_block(true);
    }
}

/// Raw DEFLATE data exactly as Java's level-6 `Deflater` produces it.
pub fn deflate_raw(raw: &[u8]) -> Vec<u8> {
    let mut d = Deflate::new(raw);
    d.deflate_slow();
    d.out
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for chunk in data.chunks(5552) {
        for &x in chunk {
            a += x as u32;
            b += a;
        }
        a %= 65521;
        b %= 65521;
    }
    (b << 16) | a
}

/// `java.util.zip.CRC32`
pub fn crc32(data: &[u8]) -> u32 {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        let mut t = [0u32; 256];
        for (n, e) in t.iter_mut().enumerate() {
            let mut c = n as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xedb88320 ^ (c >> 1) } else { c >> 1 };
            }
            *e = c;
        }
        t
    });
    let mut c = !0u32;
    for &b in data {
        c = table[((c ^ b as u32) & 0xff) as usize] ^ (c >> 8);
    }
    !c
}

/// `DeflaterOutputStream` with a default `Deflater`: zlib header, level-6 data, Adler-32.
pub fn zlib(raw: &[u8]) -> io::Result<Vec<u8>> {
    let mut out = vec![0x78, 0x9c];
    out.extend(deflate_raw(raw));
    out.extend(adler32(raw).to_be_bytes());
    Ok(out)
}

/// `GZIPOutputStream`: Java's fixed ten-byte header, raw level-6 data, CRC-32 and size.
pub fn gzip(raw: &[u8]) -> io::Result<Vec<u8>> {
    let mut out = vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 0];
    out.extend(deflate_raw(raw));
    out.extend(crc32(raw).to_le_bytes());
    out.extend((raw.len() as u32).to_le_bytes());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    fn inputs() -> Vec<Vec<u8>> {
        let mut r = crate::JavaRandom::new(7);
        let mut v = vec![vec![], vec![0], vec![1, 2], b"abcabcabcabc".to_vec(), vec![0; 100000]];
        for n in 0..40 {
            let len = r.next_int(if n < 20 { 3000 } else { 200000 }) as usize;
            let mut b = vec![0u8; len];
            for j in 0..len {
                b[j] = match n % 4 {
                    0 => r.next_int_unbounded() as u8,
                    1 if j > 0 && r.next_int(4) != 0 => b[j - 1 - r.next_int(j.min(40000) as i32) as usize],
                    2 => (r.next_int(8) == 0) as u8 * r.next_int(5) as u8,
                    _ => r.next_int(20) as u8,
                };
            }
            v.push(b);
        }
        v
    }
    #[test]
    fn output_inflates_back_to_the_input() {
        for raw in inputs() {
            let z = zlib(&raw).unwrap();
            let mut back = vec![];
            flate2::read::ZlibDecoder::new(&z[..]).read_to_end(&mut back).unwrap();
            assert_eq!(back, raw);
            let g = gzip(&raw).unwrap();
            let mut back = vec![];
            flate2::read::GzDecoder::new(&g[..]).read_to_end(&mut back).unwrap();
            assert_eq!(back, raw);
        }
    }
}
