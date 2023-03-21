use std::{fmt::Display, usize};

macro_rules! parent {
    ($x:expr) => {
        ($x - 1) / 2
    };
}

pub struct Heap<T>
where
    T: PartialOrd + Display,
{
    queue: Vec<T>,
}

impl<T> Display for Heap<T>
where
    T: PartialOrd + Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str_child(0, "".to_owned()))
    }
}

impl<T> Heap<T>
where
    T: PartialOrd + Display + Copy,
{
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn str_child(&self, pos: usize, mut spaces: String) -> String {
        if pos > self.queue.len() - 1 {
            return "".to_owned();
        }

        let mut content = format!("{}{}\n", spaces, self.queue[pos]);

        spaces.push('\t');

        content.push_str(&self.str_child(pos * 2 + 2, spaces.clone()));
        content.push_str(&self.str_child(pos * 2 + 1, spaces));

        return content;
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn insert(&mut self, value: T) {
        self.queue.push(value);

        if self.queue.len() == 1 {
            return;
        }

        self.sift_up(self.queue.len() - 1, value);
    }

    fn sift_up(&mut self, pos: usize, value: T) {
        let father_pos = parent!(pos);

        if self.queue[father_pos] < value {
            self.queue[pos] = self.queue[father_pos];
            self.queue[father_pos] = value;

            if father_pos > 0 {
                self.sift_up(father_pos, value);
            }
        }
    }

    pub fn get(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        let val = self.queue[0];

        self.sift_down(0);
        self.queue.pop();

        return Some(val);
    }

    fn sift_down(&mut self, pos: usize) {
        let mut pos_child = 2 * pos + 1;

        if pos_child > self.queue.len() - 1 {
            return;
        }

        let pos_right = pos_child + 1;

        if pos_right < self.queue.len() && self.queue[pos_right] > self.queue[pos_child] {
            pos_child = pos_right;
        }

        self.queue[pos] = self.queue[pos_child];
        self.sift_down(pos_child);
    }
}

impl<T> From<Vec<T>> for Heap<T>
where
    T: PartialOrd + Display + Copy,
{
    fn from(value: Vec<T>) -> Self {
        let mut heap = Self { queue: value };

        for i in (0..=parent!(heap.len() - 1)).rev() {
            heap.sift_down(i);
        }

        heap
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Display, vec};

    use crate::Heap;
    use rand::Rng;

    impl<T> Heap<T>
    where
        T: PartialOrd + Display + Copy,
    {
        fn check(&self) -> bool {
            return self.queue.is_empty() || self.check_child(0);
        }

        fn check_child(&self, pos: usize) -> bool {
            let p_left = 2 * pos + 1;

            if p_left > self.queue.len() - 1 {
                return true;
            }

            if self.queue[p_left] > self.queue[pos] {
                return false;
            }

            let p_right = p_left + 1;

            if p_right > self.queue.len() - 1 {
                return true;
            }

            if self.queue[p_right] > self.queue[pos] {
                return false;
            }

            return self.check_child(p_left) && self.check_child(p_right);
        }
    }

    #[test]
    fn insert_remove() {
        let mut heap = Heap::new();
        for _ in 0..1000 {
            heap.insert(rand::thread_rng().gen_range(1..=10000));
            assert!(heap.check());
        }

        while let Some(_) = heap.get() {
            assert!(heap.check());
        }
    }

    #[test]
    fn remove_empty() {
        let heap: Heap<usize> = Heap::new();
        assert!(heap.check());
    }

    #[test]
    fn heapsort() {
        let heap: Heap<_> = vec![
            974, 707, 699, 483, 588, 592, 234, 129, 119, 363, 328, 57, 247, 548, 17, 23, 101,
        ]
        .into();

        assert!(heap.check());
    }

    #[test]
    fn parent() {
        let parent = parent!(10);

        assert_eq!(parent, 4);
    }
}
