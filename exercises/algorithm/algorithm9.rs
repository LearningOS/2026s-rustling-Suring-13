/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 1. 将新元素放到向量末尾（堆的最后一个叶子节点）
        self.items.push(value);
        self.count += 1;

        // 2. 上浮调整：让新元素"冒泡"到正确位置
        let mut current_idx = self.count; // 新元素的初始索引（从1开始）
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            // 若当前元素优先级高于父节点（满足比较器），则交换父子节点
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx; // 继续向上比较
            } else {
                break; // 优先级不高于父节点，位置正确，停止上浮
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 1. 若只有左子节点，直接返回左子节点索引
        if right_idx > self.count {
            return left_idx;
        }

        // 2. 比较左右子节点，返回优先级更高的那个（满足比较器的子节点）
        let left_val = &self.items[left_idx];
        let right_val = &self.items[right_idx];
        if (self.comparator)(left_val, right_val) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None; // 堆为空，返回 None
        }

        // 1. 取出堆顶元素（索引1，优先级最高）
        let top = self.items.swap_remove(1); // swap_remove 高效移除并返回元素（用末尾元素填充）
        self.count -= 1;

        // 2. 下沉调整：让新的堆顶元素"下沉"到正确位置
        let mut current_idx = 1;
        while self.children_present(current_idx) {
            let child_idx = self.smallest_child_idx(current_idx); // 找到优先级最高的子节点
                                                                  // 若当前元素优先级低于子节点，交换两者
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx; // 继续向下比较
            } else {
                break; // 优先级不低于子节点，位置正确，停止下沉
            }
        }

        Some(top) // 返回弹出的堆顶元素
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
