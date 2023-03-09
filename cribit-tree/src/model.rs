use crate::bit_operations::{does_prefix_match, get_bit_u};

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub lower_value: V,
    pub higher_value: V,
    pub lower_node: Option<Box<Node<K, V>>>,
    pub higher_node: Option<Box<Node<K, V>>>,
    pub lower_position: K,
    pub higher_position: K,
    pub pos_diff: u32,
}

impl<K: num_traits::PrimInt, V> Node<K, V> {
    pub const fn new(
        lower_value: V,
        higher_value: V,
        lower_position: K,
        higher_position: K,
        pos_diff: u32,
    ) -> Self {
        Self {
            lower_value,
            higher_value,
            lower_position,
            higher_position,
            pos_diff,
            lower_node: None,
            higher_node: None,
        }
    }

    pub fn validate(&self, first_bit_of_node: u32, prefix: u32) -> bool {
        if self.pos_diff < first_bit_of_node {
            return false;
        }

        if self.lower_node.is_some() {
            if !does_prefix_match(
                self.pos_diff - 1,
                self.lower_position.to_u32().unwrap_or_default(),
                prefix,
            ) {
                return false;
            }
            if let Some(lower_node) = self.lower_node.as_ref() {
                return lower_node.validate(
                    self.pos_diff + 1,
                    self.lower_position.to_u32().unwrap_or_default(),
                );
            }
        }
        if self.higher_node.is_some() {
            if !does_prefix_match(
                self.pos_diff - 1,
                self.higher_position.to_u32().unwrap_or_default(),
                prefix,
            ) {
                return false;
            }
            if let Some(higher_node) = self.higher_node.as_ref() {
                return higher_node.validate(
                    self.pos_diff + 1,
                    self.higher_position.to_u32().unwrap_or_default(),
                );
            }
        }
        true
    }
}

impl<K: num_traits::PrimInt, V> Node<K, V> {
    pub fn new_num(
        lower_value: V,
        lower_position: K,
        higher_value: V,
        higher_position: K,
    ) -> Option<Self> {
        if lower_position == higher_position {
            None
        } else {
            let leading_zeros = (lower_position ^ higher_position).leading_zeros();
            if get_bit_u(higher_position.to_u32().unwrap_or_default(), leading_zeros) {
                Some(Self {
                    lower_value,
                    higher_value,
                    lower_node: None,
                    higher_node: None,
                    lower_position,
                    higher_position,
                    pos_diff: leading_zeros,
                })
            } else {
                Some(Self {
                    lower_value: higher_value,
                    higher_value: lower_value,
                    lower_node: None,
                    higher_node: None,
                    lower_position: higher_position,
                    higher_position: lower_position,
                    pos_diff: leading_zeros,
                })
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct AtomicInfo<K, V>
where
    V: Clone,
{
    pub root: Option<Node<K, V>>,
    pub root_key: K,
    pub root_value: Option<V>,
    pub len: usize,
}

impl<K, V: Clone> AtomicInfo<K, V> {
    pub const fn new(root_key: K, root_value: V) -> Self {
        Self {
            root: None,
            len: 1,
            root_key,
            root_value: Some(root_value),
        }
    }
}
