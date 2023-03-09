use crate::{
    bit_operations::{does_prefix_match, extract_prefix, get_bit_u},
    model::{AtomicInfo, Node},
};

// const DEPTH: usize = 64;

pub struct CriBit64<V: Clone> {
    info: Option<AtomicInfo<u32, V>>,
}

impl<V: Clone> Default for CriBit64<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone> CriBit64<V> {
    pub const fn new() -> Self {
        Self { info: None }
    }

    pub fn validate(&self) -> bool {
        if let Some(info) = self.info.clone() {
            let root_key = info.root_key;
            info.root.map_or(false, |node| node.validate(0, root_key))
        } else {
            true
        }
    }

    fn update_parent_after_remove(
        &mut self,
        parent: &mut Option<Node<u32, V>>,
        new_position: u32,
        new_value: V,
        new_sub: Option<Node<u32, V>>,
        is_parent_high: bool,
    ) {
        let mut new_position = new_position;
        new_position = if new_sub.is_none() {
            new_position
        } else {
            extract_prefix(new_position, new_sub.clone().unwrap().pos_diff - 1)
        };
        let mut_info = &mut self.info;
        if let Some(info) = mut_info {
            info.len -= 1;
        }

        if let (Some(info), true) = (mut_info, parent.is_none()) {
            info.root_key = new_position;
            info.root_value = Some(new_value);
            info.root = new_sub;
        } else if let Some(parent) = parent {
            if is_parent_high {
                parent.higher_position = new_position;
                parent.higher_value = new_value;
                parent.higher_node = new_sub.map(Box::new);
            } else {
                parent.lower_position = new_position;
                parent.lower_value = new_value;
                parent.lower_node = new_sub.map(Box::new);
            }
        }
    }
}

impl<V: Clone> crate::cbkd::CritBit<V> for CriBit64<V> {
    type Key = u32;

    fn insert(&mut self, key: Self::Key, value: V) -> Option<V> {
        if self.info.is_none() {
            self.info = Some(AtomicInfo::new(key, value));
            None
        } else if let Some(info) = &mut self.info {
            let info_clone = info.clone();
            let node_2 = Node::new_num(
                value.clone(),
                key,
                info_clone.root_value.clone().unwrap(),
                info_clone.root_key,
            );

            if let Some(new_node) = node_2 {
                let pos_diff = new_node.pos_diff - 1;

                info.root = Some(new_node);
                info.root_key = extract_prefix(key, pos_diff);
                info.root_value = None;
                info.len += 1;

                None
            } else {
                let old_value = info_clone.root_value;
                info.root_value = Some(value);
                old_value
            }
        } else {
            None
        }

        // Node<V> n = info.root;
        // int parentPos_diff = -1;
        // long prefix = info.rootKey;
        // Node<V> parent = null;
        // boolean isCurrentChildLo = false;
        // while (true) {
        // 	if (parentPos_diff+1 != n.pos_diff) {
        // 		//split in prefix?
        // 		int pos_diff = compare(key, prefix);
        // 		if (pos_diff < n.pos_diff && pos_diff != -1) {
        // 			Node<V> newSub;
        // 			long subPrefix = extractPrefix(prefix, pos_diff-1);
        // 			if (BitTools.getBit(key, pos_diff)) {
        // 				newSub = new Node<V>(prefix, null, key, val, pos_diff);
        // 				newSub.lo = n;
        // 			} else {
        // 				newSub = new Node<V>(key, val, prefix, null, pos_diff);
        // 				newSub.hi = n;
        // 			}
        // 			if (parent == null) {
        // 				info.rootKey = subPrefix;
        // 				info.root = newSub;
        // 			} else if (isCurrentChildLo) {
        // 				parent.loPost = subPrefix;
        // 				parent.lo = newSub;
        // 			} else {
        // 				parent.hiPost = subPrefix;
        // 				parent.hi = newSub;
        // 			}
        // 			info.size++;
        // 			return null;
        // 		}
        // 	}

        // 	//prefix matches, so now we check sub-nodes and postfixes
        // 	if (BitTools.getBit(key, n.pos_diff)) {
        // 		if (n.hi != null) {
        // 			prefix = n.hiPost;
        // 			parent = n;
        // 			n = n.hi;
        // 			isCurrentChildLo = false;
        // 		} else {
        // 			Node<V> n2 = createNode(key, val, n.hiPost, n.hiVal);
        // 			if (n2 == null) {
        // 				V prev = n.hiVal;
        // 				n.hiVal = val;
        // 				return prev;
        // 			}
        // 			n.hi = n2;
        // 			n.hiPost = extractPrefix(key, n2.pos_diff-1);
        // 			n.hiVal = null;
        // 			info.size++;
        // 			return null;
        // 		}
        // 	} else {
        // 		if (n.lo != null) {
        // 			prefix = n.loPost;
        // 			parent = n;
        // 			n = n.lo;
        // 			isCurrentChildLo = true;
        // 		} else {
        // 			Node<V> n2 = createNode(key, val, n.loPost, n.loVal);
        // 			if (n2 == null) {
        // 				V prev = n.loVal;
        // 				n.loVal = val;
        // 				return prev;
        // 			}
        // 			n.lo = n2;
        // 			n.loPost = extractPrefix(key, n2.pos_diff-1);
        // 			n.loVal = null;
        // 			info.size++;
        // 			return null;
        // 		}
        // 	}
        // 	parentPos_diff = n.pos_diff;
        // }
    }

    fn contains(&self, key: Self::Key) -> bool {
        if self.info.is_none() {
            return false;
        }
        if let Some(info) = self.info.as_ref() {
            if info.len == 1 {
                return key == info.root_key;
            }
            let mut node = info.root.as_ref().unwrap();
            let mut prefix = info.root_key;

            while does_prefix_match(node.pos_diff, key, prefix) {
                if get_bit_u(key, node.pos_diff) {
                    if let Some(high_node) = node.higher_node.as_ref() {
                        prefix = node.higher_position;
                        node = high_node;
                        continue;
                    }
                    return key == node.higher_position;
                }
                if let Some(lower_node) = node.lower_node.as_ref() {
                    prefix = node.lower_position;
                    node = lower_node;
                    continue;
                }
                return key == node.lower_position;
            }
        }
        false
    }

    fn remove(&mut self, key: Self::Key) -> Option<V> {
        let mut info = self.info.as_mut()?;
        let root_value = info.clone().root_value;
        let root_node = info.clone().root;

        if info.len == 1 && key == info.root_key {
            info.len -= 1;
            info.root_key = 0;
            let previous_value = root_value;
            info.root_value = None;
            return previous_value;
        }

        let mut node = root_node;
        let mut parent: Option<Node<u32, V>> = None;
        let mut is_parent_high = false;
        let mut prefix = info.root_key;
        while node.clone().is_some()
            && does_prefix_match(node.clone().unwrap().pos_diff, key, prefix)
        {
            //prefix matches, so now we check sub-nodes and postfixes
            if get_bit_u(key, node.clone().unwrap().pos_diff) {
                if node.clone().unwrap().higher_node.is_some() {
                    is_parent_high = true;
                    prefix = node.clone().unwrap().higher_position;
                    parent = node.clone();
                    node = node.unwrap().higher_node.map(|n| *n);
                    continue;
                }
                if key != node.clone().unwrap().higher_position {
                    return None;
                }
                //match! --> delete node
                //replace data in parent node
                self.update_parent_after_remove(
                    &mut parent,
                    node.clone().unwrap().lower_position,
                    node.clone().unwrap().lower_value,
                    node.clone().unwrap().lower_node.map(|n| *n),
                    is_parent_high,
                );
                return Some(node.unwrap().higher_value);
            } else if node.clone().unwrap().lower_node.is_some() {
                is_parent_high = false;
                prefix = node.clone().unwrap().lower_position;
                parent = node.clone();
                node = node.unwrap().lower_node.map(|n| *n);
                continue;
            }
            if key != node.clone().unwrap().lower_position {
                return None;
            }
            //match! --> delete node
            //replace data in parent node
            //for new prefixes...
            self.update_parent_after_remove(
                &mut parent,
                node.clone().unwrap().higher_position,
                node.clone().unwrap().higher_value,
                node.clone().unwrap().higher_node.map(|n| *n),
                is_parent_high,
            );
            return Some(node.unwrap().lower_value);
        }
        None
    }

    fn remove_kv(&mut self, key: Self::Key) -> Option<(Self::Key, V)> {
        let mut info = self.info.as_mut()?;
        let root_value = info.clone().root_value;
        let root_key = info.clone().root_key;
        let root_node = info.clone().root;

        if info.len == 1 && key == info.root_key {
            info.len -= 1;
            info.root_key = 0;
            let previous_value = root_value;
            info.root_value = None;
            if let Some(prev) = previous_value {
                return Some((root_key, prev));
            }
            return None;
        }

        let mut node = root_node;
        let mut parent: Option<Node<u32, V>> = None;
        let mut is_parent_high = false;
        let mut prefix = info.root_key;
        while node.clone().is_some()
            && does_prefix_match(node.clone().unwrap().pos_diff, key, prefix)
        {
            //prefix matches, so now we check sub-nodes and postfixes
            if get_bit_u(key, node.clone().unwrap().pos_diff) {
                if node.clone().unwrap().higher_node.is_some() {
                    is_parent_high = true;
                    prefix = node.clone().unwrap().higher_position;
                    parent = node.clone();
                    node = node.unwrap().higher_node.map(|n| *n);
                    continue;
                }
                if key != node.clone().unwrap().higher_position {
                    return None;
                }
                //match! --> delete node
                //replace data in parent node
                self.update_parent_after_remove(
                    &mut parent,
                    node.clone().unwrap().lower_position,
                    node.clone().unwrap().lower_value,
                    node.clone().unwrap().lower_node.map(|n| *n),
                    is_parent_high,
                );
                return Some((
                    node.clone().unwrap().higher_position,
                    node.unwrap().higher_value,
                ));
            } else if node.clone().unwrap().lower_node.is_some() {
                is_parent_high = false;
                prefix = node.clone().unwrap().lower_position;
                parent = node.clone();
                node = node.unwrap().lower_node.map(|n| *n);
                continue;
            }
            if key != node.clone().unwrap().lower_position {
                return None;
            }
            //match! --> delete node
            //replace data in parent node
            //for new prefixes...
            self.update_parent_after_remove(
                &mut parent,
                node.clone().unwrap().higher_position,
                node.clone().unwrap().higher_value,
                node.clone().unwrap().higher_node.map(|n| *n),
                is_parent_high,
            );
            return Some((
                node.clone().unwrap().lower_position,
                node.unwrap().lower_value,
            ));
        }
        None
    }

    fn len(&self) -> usize {
        self.info.as_ref().map_or(0, |node| node.len)
    }

    fn get(&self, key: Self::Key) -> Option<&V> {
        let info = self.info.as_ref()?;
        if info.len == 1 && key == info.root_key {
            return info.root_value.as_ref();
        }
        let mut node = info.root.as_ref().unwrap();
        let mut prefix = info.root_key;

        while does_prefix_match(node.pos_diff, key, prefix) {
            if get_bit_u(key, node.pos_diff) {
                if let Some(high_node) = node.higher_node.as_ref() {
                    prefix = node.higher_position;
                    node = high_node;
                    continue;
                }
                if key == node.higher_position {
                    return Some(&node.higher_value);
                }
            } else {
                if let Some(lower_node) = node.lower_node.as_ref() {
                    prefix = node.lower_position;
                    node = lower_node;
                    continue;
                }
                if key == node.lower_position {
                    return Some(&node.lower_value);
                }
            };
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
