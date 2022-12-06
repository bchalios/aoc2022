use std::collections::HashSet;

//  0 1 2 3 4 5 6
// [ | | | | | |x]

#[derive(Debug)]
enum QueueError {
    FullQueue,
    EmptyQueue,
}

/// A First-In-First-Out fixed-size queue
struct Queue<T>
where
    T: Default + Copy,
{
    data: Vec<T>,
    head: usize,
    tail: usize,
    raw_size: usize,
}

impl<T> Queue<T>
where
    T: Default + Copy,
{
    fn new(size: usize) -> Self {
        Self {
            data: vec![T::default(); size + 1],
            head: 0,
            tail: 0,
            raw_size: size + 1,
        }
    }

    fn len(&self) -> usize {
        (self.tail - self.head) % (self.raw_size + 1)
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn next_tail(&self) -> usize {
        (self.tail + 1) % self.raw_size
    }

    fn is_full(&self) -> bool {
        self.next_tail() == self.head
    }

    fn push(&mut self, item: T) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::FullQueue);
        }

        self.data[self.tail] = item;
        self.tail = self.next_tail();
        Ok(())
    }

    fn pop(&mut self) -> Result<T, QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueue);
        }

        let pos = self.head;
        let item = self.data[pos];
        self.head = (pos + 1) % self.raw_size;
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::<u32>::new(5);
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert!(!queue.is_full());

        assert!(matches!(queue.pop(), Err(QueueError::EmptyQueue)));
        assert!(matches!(queue.push(42), Ok(())));
        assert!(!queue.is_empty());
        assert!(!queue.is_full());

        assert!(matches!(queue.pop(), Ok(42)));

        assert!(matches!(queue.push(1), Ok(())));
        assert!(matches!(queue.push(2), Ok(())));
        assert!(matches!(queue.push(3), Ok(())));
        assert!(matches!(queue.push(4), Ok(())));
        assert!(matches!(queue.push(5), Ok(())));
        assert!(queue.is_full());
        assert!(!queue.is_empty());
        assert!(matches!(queue.push(6), Err(QueueError::FullQueue)));

        assert!(matches!(queue.pop(), Ok(1)));
        assert!(matches!(queue.pop(), Ok(2)));
        assert!(matches!(queue.pop(), Ok(3)));
        assert!(matches!(queue.pop(), Ok(4)));
        assert!(matches!(queue.pop(), Ok(5)));
        assert!(matches!(queue.pop(), Err(QueueError::EmptyQueue)));
    }
}

fn find_preamble(input: &mut str) -> Option<usize> {
    let mut pos = 14;

    let inter = input.chars().collect::<Vec<char>>();
    for chunk in inter.windows(14) {
        let mut window = HashSet::new();
        let mut is_preamble = true;
        for c in chunk {
            if !window.insert(c) {
                is_preamble = false;
                break;
            }
        }

        if is_preamble {
            return Some(pos);
        } else {
            pos += 1;
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{}", find_preamble(input.as_mut_str()).unwrap());
}
