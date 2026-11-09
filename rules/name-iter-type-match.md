# name-iter-type-match

> Name iterator types after their source method

## Why It Matters

Iterator types should match the method that creates them. `iter()` returns `Iter`, `into_iter()` returns `IntoIter`, `keys()` returns `Keys`. This naming pattern is established by the standard library and makes types predictable.

## Standard Library Pattern

```rust
use std::collections::HashMap;

let mut values = vec![1, 2];
let iter: std::slice::Iter<'_, i32> = values.iter();
assert_eq!(iter.copied().sum::<i32>(), 3);
let iter_mut: std::slice::IterMut<'_, i32> = values.iter_mut();
for value in iter_mut { *value += 1; }
let owned: std::vec::IntoIter<i32> = values.into_iter();
assert_eq!(owned.collect::<Vec<_>>(), [2, 3]);

let map = HashMap::from([("a", 1)]);
let keys: std::collections::hash_map::Keys<'_, &str, i32> = map.keys();
assert_eq!(keys.count(), 1);
```

## Implementation

```rust
mod my_collection {
    pub struct MyCollection<T> {
        items: Vec<T>,
    }
    
    // Iterator types in same module
    pub struct Iter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }
    
    pub struct IterMut<'a, T> {
        inner: std::slice::IterMut<'a, T>,
    }
    
    pub struct IntoIter<T> {
        inner: std::vec::IntoIter<T>,
    }
    
    impl<T> MyCollection<T> {
        pub fn iter(&self) -> Iter<'_, T> {
            Iter { inner: self.items.iter() }
        }
        
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut { inner: self.items.iter_mut() }
        }
    }
    
    impl<T> IntoIterator for MyCollection<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        
        fn into_iter(self) -> IntoIter<T> {
            IntoIter { inner: self.items.into_iter() }
        }
    }
    
    // Implement Iterator for each type
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }
    
    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }
    
    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }
}
```

## Naming Convention

| Method | Iterator Type |
|--------|---------------|
| `iter()` | `Iter` |
| `iter_mut()` | `IterMut` |
| `into_iter()` | `IntoIter` |
| `keys()` | `Keys` |
| `values()` | `Values` |
| `values_mut()` | `ValuesMut` |
| `drain()` | `Drain` |
| `chunks()` | `Chunks` |
| `windows()` | `Windows` |

## Custom Iterator Methods

```rust
type NodeId = usize;
struct Graph { nodes: Vec<NodeId>, edges: Vec<(NodeId, NodeId)> }
pub struct Nodes<'a> { inner: std::slice::Iter<'a, NodeId> }
pub struct Edges<'a> { inner: std::slice::Iter<'a, (NodeId, NodeId)> }
pub struct Neighbors<'a> { inner: Box<dyn Iterator<Item = NodeId> + 'a> }
impl Graph {
    fn nodes(&self) -> Nodes<'_> { Nodes { inner: self.nodes.iter() } }
    fn edges(&self) -> Edges<'_> { Edges { inner: self.edges.iter() } }
    fn neighbors(&self, node: NodeId) -> Neighbors<'_> {
        Neighbors { inner: Box::new(self.edges.iter().filter_map(move |&(from, to)| (from == node).then_some(to))) }
    }
}
impl<'a> Iterator for Nodes<'a> {
    type Item = &'a NodeId;
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}
impl<'a> Iterator for Edges<'a> {
    type Item = &'a (NodeId, NodeId);
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}
impl Iterator for Neighbors<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}
```

## Bad

```rust
// Mismatched names
impl MyCollection<T> {
    fn iter(&self) -> MyCollectionIterator<'_, T> { }  // Should be Iter
    fn keys(&self) -> KeyIterator<'_, K> { }           // Should be Keys
}

// Generic names that don't match method
pub struct Iterator<T>;  // Conflicts with std::iter::Iterator
pub struct I<T>;         // Too cryptic
```

## See Also

- [name-iter-convention](./name-iter-convention.md) - iter/iter_mut/into_iter
- [name-iter-method](./name-iter-method.md) - Iterator method names
- [api-common-traits](./api-common-traits.md) - Implementing common traits
