mod heap_sort;

pub struct Node<K, V> {
    key: K,
    value: V,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node { key, value }
    }

    pub fn get(&self) -> &V {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<K: PartialEq, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: PartialOrd, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

pub struct Map<K, V, const MAX_LEN: usize> {
    map: [Option<Node<K, V>>; MAX_LEN],
    len: usize,
}

impl<K: PartialOrd, V, const MAX_LEN: usize> Map<K, V, MAX_LEN> {
    pub fn new() -> Self {
        Map {
            map: [const { None }; MAX_LEN],
            len: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<()> {
        if self.len >= MAX_LEN {
            return None;
        }

        self.map[self.len].replace(Node::new(key, value));

        self.len += 1;

        // Sort the map after insertion
        heap_sort::heap_sort(&mut self.map[..self.len]);

        Some(())
    }

    // get method to retrieve a value by key using binary search
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut left = 0;
        let mut right = self.len;

        while left < right {
            let mid = (left + right) / 2;
            if &self.map[mid].as_ref()?.key == key {
                return Some(self.map[mid].as_ref()?.get());
            } else if &self.map[mid].as_ref()?.key < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        None
    }

    // get mut method to retrieve a value by key using binary search
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut left = 0;
        let mut right = self.len;

        while left < right {
            let mid = (left + right) / 2;
            if &self.map[mid].as_ref()?.key == key {
                return Some(self.map[mid].as_mut()?.get_mut());
            } else if &self.map[mid].as_ref()?.key < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = Map::<i32, i32, 10>::new();
        map.insert(5, 10);
        map.insert(3, 20);
        map.insert(8, 30);
        map.insert(2, 40);
        map.insert(7, 50);
        map.insert(6, 60);
        map.insert(4, 70);
        map.insert(0, 90);
        map.insert(9, 100);
        map.insert(1, 110);
        assert_eq!(map.insert(20, 3), None);

        assert_eq!(map.get(&5), Some(&10));
        assert_eq!(map.get(&3), Some(&20));
        assert_eq!(map.get(&8), Some(&30));
        assert_eq!(map.get(&2), Some(&40));
        assert_eq!(map.get(&7), Some(&50));
        assert_eq!(map.get(&6), Some(&60));
        assert_eq!(map.get(&4), Some(&70));
        assert_eq!(map.get(&0), Some(&90));
        assert_eq!(map.get(&9), Some(&100));
        assert_eq!(map.get(&1), Some(&110));
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn test_map_mut() {
        let mut map = Map::<i32, i32, 10>::new();
        map.insert(5, 10);
        map.insert(3, 20);
        map.insert(8, 30);
        map.insert(2, 40);
        map.insert(7, 50);
        map.insert(6, 60);
        map.insert(4, 70);
        map.insert(0, 90);
        map.insert(9, 100);
        map.insert(1, 110);

        if let Some(value) = map.get_mut(&5) {
            *value += 5;
        }

        if let Some(value) = map.get_mut(&3) {
            *value += 10;
        }
        if let Some(value) = map.get_mut(&8) {
            *value += 15;
        }
        if let Some(value) = map.get_mut(&2) {
            *value += 20;
        }
        if let Some(value) = map.get_mut(&7) {
            *value += 25;
        }
        if let Some(value) = map.get_mut(&6) {
            *value += 30;
        }
        if let Some(value) = map.get_mut(&4) {
            *value += 35;
        }
        if let Some(value) = map.get_mut(&0) {
            *value += 40;
        }
        if let Some(value) = map.get_mut(&9) {
            *value += 45;
        }
        if let Some(value) = map.get_mut(&1) {
            *value += 50;
        }

        assert_eq!(map.get(&5), Some(&15));
        assert_eq!(map.get(&3), Some(&30));
        assert_eq!(map.get(&8), Some(&45));
        assert_eq!(map.get(&2), Some(&60));
        assert_eq!(map.get(&7), Some(&75));
        assert_eq!(map.get(&6), Some(&90));
        assert_eq!(map.get(&4), Some(&105));
        assert_eq!(map.get(&0), Some(&130));
        assert_eq!(map.get(&9), Some(&145));
        assert_eq!(map.get(&1), Some(&160));
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn test_node() {
        let node = Node::new(1, "value");
        assert_eq!(node.get(), &"value");
    }

    #[test]
    fn test_node_mut() {
        let mut node = Node::new(1, "value");
        *node.get_mut() = "new_value";
        assert_eq!(node.get(), &"new_value");
    }

    #[test]
    fn test_node_partial_eq() {
        let node1 = Node::new(1, "value");
        let node2 = Node::new(1, "value1");
        let node3 = Node::new(2, "value2");

        assert!(node1.eq(&node2));
        assert!(!node1.eq(&node3));
        assert!(node2.ne(&node3));
    }

    #[test]
    fn test_node_partial_ord() {
        let node1 = Node::new(1, "value");
        let node2 = Node::new(1, "value1");
        let node3 = Node::new(2, "value2");

        assert_eq!(node1.partial_cmp(&node2), Some(core::cmp::Ordering::Equal));
        assert_eq!(node1.partial_cmp(&node3), Some(core::cmp::Ordering::Less));
        assert_eq!(
            node3.partial_cmp(&node1),
            Some(core::cmp::Ordering::Greater)
        );
    }
}
