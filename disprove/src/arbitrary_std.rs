//! Arbitrary implementations for standard library types.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{CString, OsString};
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
};
use std::ops::{Range, RangeInclusive};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use crate::arbitrary::Arbitrary;
use crate::gen::Gen;
use crate::shrink::empty_shrinker;

// -- Vec<T> --

impl<T: Arbitrary> Arbitrary for Vec<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| T::arbitrary(g)).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let v = self.clone();
        if v.is_empty() {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push(Vec::new());
        for i in 0..v.len() {
            let mut shorter = v.clone();
            shorter.remove(i);
            candidates.push(shorter);
        }
        for i in 0..v.len() {
            for smaller in v[i].shrink() {
                let mut shrunk = v.clone();
                shrunk[i] = smaller;
                candidates.push(shrunk);
            }
        }
        Box::new(candidates.into_iter())
    }
}

// -- HashMap<K, V> --

impl<K: Arbitrary + Eq + Hash, V: Arbitrary> Arbitrary for HashMap<K, V> {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| (K::arbitrary(g), V::arbitrary(g))).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let entries: Vec<_> = self.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        if entries.is_empty() {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push(HashMap::new());
        for i in 0..entries.len() {
            let shrunk: HashMap<K, V> = entries
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, (k, v))| (k.clone(), v.clone()))
                .collect();
            candidates.push(shrunk);
        }
        Box::new(candidates.into_iter())
    }
}

// -- HashSet<T> --

impl<T: Arbitrary + Eq + Hash> Arbitrary for HashSet<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| T::arbitrary(g)).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let entries: Vec<_> = self.iter().cloned().collect();
        if entries.is_empty() {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push(HashSet::new());
        for i in 0..entries.len() {
            let shrunk: HashSet<T> = entries
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, v)| v.clone())
                .collect();
            candidates.push(shrunk);
        }
        Box::new(candidates.into_iter())
    }
}

// -- BTreeMap<K, V> --

impl<K: Arbitrary + Ord, V: Arbitrary> Arbitrary for BTreeMap<K, V> {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| (K::arbitrary(g), V::arbitrary(g))).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let entries: Vec<_> = self.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        if entries.is_empty() {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push(BTreeMap::new());
        for i in 0..entries.len() {
            let shrunk: BTreeMap<K, V> = entries
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, (k, v))| (k.clone(), v.clone()))
                .collect();
            candidates.push(shrunk);
        }
        Box::new(candidates.into_iter())
    }
}

// -- BTreeSet<T> --

impl<T: Arbitrary + Ord> Arbitrary for BTreeSet<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| T::arbitrary(g)).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let entries: Vec<_> = self.iter().cloned().collect();
        if entries.is_empty() {
            return empty_shrinker();
        }
        let mut candidates = Vec::new();
        candidates.push(BTreeSet::new());
        for i in 0..entries.len() {
            let shrunk: BTreeSet<T> = entries
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, v)| v.clone())
                .collect();
            candidates.push(shrunk);
        }
        Box::new(candidates.into_iter())
    }
}

// -- VecDeque<T> --

impl<T: Arbitrary> Arbitrary for VecDeque<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Vec::<T>::arbitrary(g).into()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let vec: Vec<T> = self.iter().cloned().collect();
        Box::new(vec.shrink().map(|v| v.into()))
    }
}

// -- LinkedList<T> --

impl<T: Arbitrary> Arbitrary for LinkedList<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Vec::<T>::arbitrary(g).into_iter().collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let vec: Vec<T> = self.iter().cloned().collect();
        Box::new(vec.shrink().map(|v| v.into_iter().collect()))
    }
}

// -- BinaryHeap<T> --

impl<T: Arbitrary + Ord> Arbitrary for BinaryHeap<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Vec::<T>::arbitrary(g).into_iter().collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let vec: Vec<T> = self.iter().cloned().collect();
        Box::new(vec.shrink().map(|v| v.into_iter().collect()))
    }
}

// -- String --

impl Arbitrary for String {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        (0..len).map(|_| char::arbitrary(g)).collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        if self.is_empty() {
            return empty_shrinker();
        }
        let chars: Vec<char> = self.chars().collect();
        let mut candidates = Vec::new();
        candidates.push(String::new());
        for i in 0..chars.len() {
            let s: String = chars
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, c)| c)
                .collect();
            candidates.push(s);
        }
        Box::new(candidates.into_iter())
    }
}

// -- CString --

impl Arbitrary for CString {
    fn arbitrary(g: &mut Gen) -> Self {
        let len = g.gen_range(0..=g.size());
        let bytes: Vec<u8> = (0..len).map(|_| g.gen_range(1u8..=255u8)).collect();
        CString::new(bytes).unwrap()
    }
}

// -- OsString --

impl Arbitrary for OsString {
    fn arbitrary(g: &mut Gen) -> Self {
        OsString::from(String::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let s = self.to_string_lossy().into_owned();
        Box::new(s.shrink().map(OsString::from))
    }
}

// -- PathBuf --

impl Arbitrary for PathBuf {
    fn arbitrary(g: &mut Gen) -> Self {
        PathBuf::from(String::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let s = self.to_string_lossy().into_owned();
        Box::new(s.shrink().map(PathBuf::from))
    }
}

// -- Smart pointers --

impl<T: Arbitrary> Arbitrary for Box<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Box::new(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((**self).shrink().map(Box::new))
    }
}

impl<T: Arbitrary> Arbitrary for Rc<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Rc::new(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((**self).shrink().map(Rc::new))
    }
}

impl<T: Arbitrary> Arbitrary for Arc<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Arc::new(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((**self).shrink().map(Arc::new))
    }
}

impl<T: Arbitrary + Copy> Arbitrary for Cell<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Cell::new(T::arbitrary(g))
    }
}

impl<T: Arbitrary> Arbitrary for RefCell<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        RefCell::new(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let val = self.borrow().clone();
        Box::new(val.shrink().map(RefCell::new))
    }
}

// -- Option<T> --

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        if g.gen_bool(0.25) {
            None
        } else {
            Some(T::arbitrary(g))
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            None => empty_shrinker(),
            Some(val) => {
                let mut candidates: Vec<Option<T>> = vec![None];
                candidates.extend(val.shrink().map(Some));
                Box::new(candidates.into_iter())
            }
        }
    }
}

// -- Result<T, E> --

impl<T: Arbitrary, E: Arbitrary> Arbitrary for Result<T, E> {
    fn arbitrary(g: &mut Gen) -> Self {
        if g.gen_bool(0.25) {
            Err(E::arbitrary(g))
        } else {
            Ok(T::arbitrary(g))
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Ok(val) => Box::new(val.shrink().map(Ok)),
            Err(val) => Box::new(val.shrink().map(Err)),
        }
    }
}

// -- Tuples up to 12 elements --
// Shrink only the first component to avoid combinatorial explosion.
// The test runner handles iterative shrinking across components.

macro_rules! arbitrary_tuple {
    // Base case for 1-tuple
    ($A:ident) => {
        impl<$A: Arbitrary> Arbitrary for ($A,) {
            fn arbitrary(g: &mut Gen) -> Self {
                ($A::arbitrary(g),)
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let val = self.0.clone();
                Box::new(val.shrink().map(|a| (a,)))
            }
        }
    };

    // 2+ tuples
    ($A:ident, $($rest:ident),+) => {
        impl<$A: Arbitrary, $($rest: Arbitrary),+> Arbitrary for ($A, $($rest,)+) {
            fn arbitrary(g: &mut Gen) -> Self {
                ($A::arbitrary(g), $($rest::arbitrary(g),)+)
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let tuple = self.clone();
                let first_shrinks: Vec<Self> = self.0.shrink().map(|a| {
                    let mut t = tuple.clone();
                    t.0 = a;
                    t
                }).collect();
                Box::new(first_shrinks.into_iter())
            }
        }

        // Recurse for fewer elements
        arbitrary_tuple!($($rest),+);
    };
}

arbitrary_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

// -- Arrays [T; N] via const generics --

impl<T: Arbitrary, const N: usize> Arbitrary for [T; N] {
    fn arbitrary(g: &mut Gen) -> Self {
        std::array::from_fn(|_| T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let arr = self.clone();
        let mut candidates = Vec::new();
        for i in 0..N {
            for smaller in arr[i].shrink() {
                let mut candidate = arr.clone();
                candidate[i] = smaller;
                candidates.push(candidate);
            }
        }
        Box::new(candidates.into_iter())
    }
}

// -- NonZero types --

macro_rules! arbitrary_nonzero_unsigned {
    ($nz:ty, $inner:ty) => {
        impl Arbitrary for $nz {
            fn arbitrary(g: &mut Gen) -> Self {
                let s = g.size();
                let upper = if s as u128 >= <$inner>::MAX as u128 {
                    <$inner>::MAX
                } else if s == 0 {
                    1
                } else {
                    s as $inner
                };
                let val = g.gen_range(1..=upper);
                <$nz>::new(val).unwrap()
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let val = self.get();
                if val == 1 {
                    return empty_shrinker();
                }
                let mut candidates = Vec::new();
                candidates.push(<$nz>::new(1).unwrap());
                let mut diff = val;
                loop {
                    diff /= 2;
                    if diff == 0 {
                        break;
                    }
                    let candidate = val - diff;
                    if candidate > 1 {
                        candidates.push(<$nz>::new(candidate).unwrap());
                    }
                }
                Box::new(candidates.into_iter())
            }
        }
    };
}

macro_rules! arbitrary_nonzero_signed {
    ($nz:ty, $inner:ty) => {
        impl Arbitrary for $nz {
            fn arbitrary(g: &mut Gen) -> Self {
                let s = g.size();
                let upper = if s as u128 >= <$inner>::MAX as u128 {
                    <$inner>::MAX
                } else if s == 0 {
                    1
                } else {
                    s as $inner
                };
                let lower = if upper == <$inner>::MAX {
                    <$inner>::MIN
                } else {
                    -upper
                };
                loop {
                    let val = g.gen_range(lower..=upper);
                    if val != 0 {
                        return <$nz>::new(val).unwrap();
                    }
                }
            }

            fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                let val = self.get();
                if val == 1 || val == -1 {
                    return empty_shrinker();
                }
                let mut candidates = Vec::new();
                candidates.push(<$nz>::new(1).unwrap());
                if val < 0 {
                    if let Some(pos) = val.checked_neg() {
                        if let Some(nz) = <$nz>::new(pos) {
                            candidates.push(nz);
                        }
                    }
                }
                let mut diff = val;
                loop {
                    diff /= 2;
                    if diff == 0 {
                        break;
                    }
                    let candidate = val - diff;
                    if candidate != 0 {
                        if let Some(nz) = <$nz>::new(candidate) {
                            candidates.push(nz);
                        }
                    }
                }
                Box::new(candidates.into_iter())
            }
        }
    };
}

arbitrary_nonzero_unsigned!(NonZeroU8, u8);
arbitrary_nonzero_unsigned!(NonZeroU16, u16);
arbitrary_nonzero_unsigned!(NonZeroU32, u32);
arbitrary_nonzero_unsigned!(NonZeroU64, u64);
arbitrary_nonzero_unsigned!(NonZeroU128, u128);
arbitrary_nonzero_unsigned!(NonZeroUsize, usize);

arbitrary_nonzero_signed!(NonZeroI8, i8);
arbitrary_nonzero_signed!(NonZeroI16, i16);
arbitrary_nonzero_signed!(NonZeroI32, i32);
arbitrary_nonzero_signed!(NonZeroI64, i64);
arbitrary_nonzero_signed!(NonZeroI128, i128);
arbitrary_nonzero_signed!(NonZeroIsize, isize);

// -- Range<T> --

impl<T: Arbitrary + PartialOrd> Arbitrary for Range<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let a = T::arbitrary(g);
        let b = T::arbitrary(g);
        if a < b { a..b } else { b..a }
    }
}

// -- RangeInclusive<T> --

impl<T: Arbitrary + PartialOrd> Arbitrary for RangeInclusive<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let a = T::arbitrary(g);
        let b = T::arbitrary(g);
        if a < b { a..=b } else { b..=a }
    }
}

// -- Wrapping<T> --

impl<T: Arbitrary> Arbitrary for Wrapping<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Wrapping(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Wrapping))
    }
}

// -- Saturating<T> --

impl<T: Arbitrary> Arbitrary for Saturating<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Saturating(T::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Saturating))
    }
}

// -- Duration --

impl Arbitrary for Duration {
    fn arbitrary(g: &mut Gen) -> Self {
        let secs = u64::arbitrary(g);
        let nanos = g.gen_range(0u32..1_000_000_000);
        Duration::new(secs, nanos)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let secs = self.as_secs();
        let nanos = self.subsec_nanos();
        if secs == 0 && nanos == 0 {
            return empty_shrinker();
        }
        let mut candidates = vec![Duration::ZERO];
        for s in secs.shrink() {
            candidates.push(Duration::new(s, nanos));
        }
        Box::new(candidates.into_iter())
    }
}

// -- IP addresses --

impl Arbitrary for Ipv4Addr {
    fn arbitrary(g: &mut Gen) -> Self {
        Ipv4Addr::new(u8::arbitrary(g), u8::arbitrary(g), u8::arbitrary(g), u8::arbitrary(g))
    }
}

impl Arbitrary for Ipv6Addr {
    fn arbitrary(g: &mut Gen) -> Self {
        Ipv6Addr::new(
            u16::arbitrary(g), u16::arbitrary(g), u16::arbitrary(g), u16::arbitrary(g),
            u16::arbitrary(g), u16::arbitrary(g), u16::arbitrary(g), u16::arbitrary(g),
        )
    }
}

impl Arbitrary for IpAddr {
    fn arbitrary(g: &mut Gen) -> Self {
        if bool::arbitrary(g) {
            IpAddr::V4(Ipv4Addr::arbitrary(g))
        } else {
            IpAddr::V6(Ipv6Addr::arbitrary(g))
        }
    }
}

impl Arbitrary for SocketAddrV4 {
    fn arbitrary(g: &mut Gen) -> Self {
        SocketAddrV4::new(Ipv4Addr::arbitrary(g), u16::arbitrary(g))
    }
}

impl Arbitrary for SocketAddrV6 {
    fn arbitrary(g: &mut Gen) -> Self {
        SocketAddrV6::new(Ipv6Addr::arbitrary(g), u16::arbitrary(g), 0, 0)
    }
}

impl Arbitrary for SocketAddr {
    fn arbitrary(g: &mut Gen) -> Self {
        if bool::arbitrary(g) {
            SocketAddr::V4(SocketAddrV4::arbitrary(g))
        } else {
            SocketAddr::V6(SocketAddrV6::arbitrary(g))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_gen() -> Gen {
        Gen::from_seed(20, 12345)
    }

    #[test]
    fn test_vec_arbitrary() {
        let mut g = test_gen();
        let v: Vec<u8> = Arbitrary::arbitrary(&mut g);
        assert!(v.len() <= 20);
    }

    #[test]
    fn test_vec_shrink() {
        let v = vec![5u32, 10, 15];
        let shrunk: Vec<Vec<u32>> = v.shrink().collect();
        assert!(shrunk.contains(&vec![]));
        assert!(shrunk.contains(&vec![10, 15]));
        assert!(shrunk.contains(&vec![5, 15]));
        assert!(shrunk.contains(&vec![5, 10]));
    }

    #[test]
    fn test_hashmap_arbitrary() {
        let mut g = test_gen();
        let m: HashMap<u8, u8> = Arbitrary::arbitrary(&mut g);
        assert!(m.len() <= 20);
    }

    #[test]
    fn test_hashset_arbitrary() {
        let mut g = test_gen();
        let s: HashSet<u8> = Arbitrary::arbitrary(&mut g);
        assert!(s.len() <= 20);
    }

    #[test]
    fn test_btreemap_arbitrary() {
        let mut g = test_gen();
        let m: BTreeMap<u8, u8> = Arbitrary::arbitrary(&mut g);
        assert!(m.len() <= 20);
    }

    #[test]
    fn test_btreeset_arbitrary() {
        let mut g = test_gen();
        let s: BTreeSet<u8> = Arbitrary::arbitrary(&mut g);
        assert!(s.len() <= 20);
    }

    #[test]
    fn test_vecdeque_arbitrary() {
        let mut g = test_gen();
        let d: VecDeque<u8> = Arbitrary::arbitrary(&mut g);
        assert!(d.len() <= 20);
    }

    #[test]
    fn test_linkedlist_arbitrary() {
        let mut g = test_gen();
        let l: LinkedList<u8> = Arbitrary::arbitrary(&mut g);
        assert!(l.len() <= 20);
    }

    #[test]
    fn test_binaryheap_arbitrary() {
        let mut g = test_gen();
        let h: BinaryHeap<u8> = Arbitrary::arbitrary(&mut g);
        assert!(h.len() <= 20);
    }

    #[test]
    fn test_string_arbitrary() {
        let mut g = test_gen();
        let s: String = Arbitrary::arbitrary(&mut g);
        assert!(s.chars().count() <= 20);
    }

    #[test]
    fn test_string_shrink() {
        let s = String::from("abc");
        let shrunk: Vec<String> = s.shrink().collect();
        assert!(shrunk.contains(&String::new()));
        assert!(shrunk.contains(&String::from("bc")));
        assert!(shrunk.contains(&String::from("ac")));
        assert!(shrunk.contains(&String::from("ab")));
    }

    #[test]
    fn test_cstring_arbitrary() {
        let mut g = test_gen();
        let _: CString = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_osstring_arbitrary() {
        let mut g = test_gen();
        let _: OsString = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_pathbuf_arbitrary() {
        let mut g = test_gen();
        let _: PathBuf = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_box_arbitrary_and_shrink() {
        let mut g = test_gen();
        let b: Box<u32> = Arbitrary::arbitrary(&mut g);
        assert!(*b <= 20);
        let b: Box<u32> = Box::new(10u32);
        let shrunk: Vec<Box<u32>> = b.shrink().collect();
        assert!(shrunk.contains(&Box::new(0)));
    }

    #[test]
    fn test_rc_arbitrary() {
        let mut g = test_gen();
        let _: Rc<u32> = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_arc_arbitrary() {
        let mut g = test_gen();
        let _: Arc<u32> = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_option_arbitrary() {
        let mut g = Gen::from_seed(100, 42);
        let mut seen_none = false;
        let mut seen_some = false;
        for _ in 0..100 {
            let v: Option<u8> = Arbitrary::arbitrary(&mut g);
            match v {
                None => seen_none = true,
                Some(_) => seen_some = true,
            }
        }
        assert!(seen_none && seen_some);
    }

    #[test]
    fn test_option_shrink() {
        let v: Option<u32> = Some(10);
        let shrunk: Vec<_> = v.shrink().collect();
        assert!(shrunk.contains(&None));
    }

    #[test]
    fn test_result_arbitrary() {
        let mut g = Gen::from_seed(100, 42);
        let mut seen_ok = false;
        let mut seen_err = false;
        for _ in 0..100 {
            let v: Result<u8, u8> = Arbitrary::arbitrary(&mut g);
            match v {
                Ok(_) => seen_ok = true,
                Err(_) => seen_err = true,
            }
        }
        assert!(seen_ok && seen_err);
    }

    #[test]
    fn test_tuple2_arbitrary() {
        let mut g = test_gen();
        let t: (u8, u8) = Arbitrary::arbitrary(&mut g);
        assert!(t.0 <= 20 && t.1 <= 20);
    }

    #[test]
    fn test_tuple3_arbitrary() {
        let mut g = test_gen();
        let _: (u8, bool, u16) = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_tuple12_arbitrary() {
        let mut g = test_gen();
        let _: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_array_arbitrary() {
        let mut g = test_gen();
        let arr: [u8; 5] = Arbitrary::arbitrary(&mut g);
        for v in &arr {
            assert!(*v <= 20);
        }
    }

    #[test]
    fn test_array_const_generic_large() {
        let mut g = Gen::from_seed(50, 42);
        let _: [u8; 64] = Arbitrary::arbitrary(&mut g);
        let _: [u8; 128] = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_nonzero_u8_arbitrary() {
        let mut g = test_gen();
        for _ in 0..100 {
            let v: NonZeroU8 = Arbitrary::arbitrary(&mut g);
            assert!(v.get() > 0);
        }
    }

    #[test]
    fn test_nonzero_i32_arbitrary() {
        let mut g = test_gen();
        for _ in 0..100 {
            let v: NonZeroI32 = Arbitrary::arbitrary(&mut g);
            assert!(v.get() != 0);
        }
    }

    #[test]
    fn test_nonzero_shrink() {
        let v = NonZeroU32::new(100).unwrap();
        let shrunk: Vec<_> = v.shrink().collect();
        assert!(!shrunk.is_empty());
        assert_eq!(shrunk[0].get(), 1);
    }

    #[test]
    fn test_wrapping_arbitrary() {
        let mut g = test_gen();
        let _: Wrapping<u32> = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_saturating_arbitrary() {
        let mut g = test_gen();
        let _: Saturating<u32> = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_duration_arbitrary() {
        let mut g = test_gen();
        let _: Duration = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_duration_shrink() {
        let d = Duration::new(100, 500_000_000);
        let shrunk: Vec<Duration> = d.shrink().collect();
        assert!(shrunk.contains(&Duration::ZERO));
    }

    #[test]
    fn test_ipv4_arbitrary() {
        let mut g = test_gen();
        let _: Ipv4Addr = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_ipv6_arbitrary() {
        let mut g = test_gen();
        let _: Ipv6Addr = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_ipaddr_both_variants() {
        let mut g = Gen::from_seed(100, 42);
        let mut seen_v4 = false;
        let mut seen_v6 = false;
        for _ in 0..100 {
            match IpAddr::arbitrary(&mut g) {
                IpAddr::V4(_) => seen_v4 = true,
                IpAddr::V6(_) => seen_v6 = true,
            }
        }
        assert!(seen_v4 && seen_v6);
    }

    #[test]
    fn test_socketaddr_arbitrary() {
        let mut g = test_gen();
        let _: SocketAddr = Arbitrary::arbitrary(&mut g);
    }

    #[test]
    fn test_range_arbitrary() {
        let mut g = test_gen();
        let r: Range<u8> = Arbitrary::arbitrary(&mut g);
        assert!(r.start <= r.end);
    }

    #[test]
    fn test_range_inclusive_arbitrary() {
        let mut g = test_gen();
        let r: RangeInclusive<u8> = Arbitrary::arbitrary(&mut g);
        assert!(r.start() <= r.end());
    }
}
