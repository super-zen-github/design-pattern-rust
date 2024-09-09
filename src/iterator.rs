pub trait Iterator<'a, T> {
    fn next(&mut self) -> Option<&'a T>;
    fn has_next(&self) -> bool;
}

pub trait List<T> {
    fn push(&mut self, t: T);
    fn pop(&mut self) -> T;
    fn index_at(&self, i: usize) -> &T;
    fn length(&self) -> usize;
}

pub struct ConcreteList<T> {
    list: Vec<T>
}

pub struct ConcreteIterator<'a, L, T> 
where L: List<T> 
{
    c_list: &'a L,
    current_idx: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ConcreteList<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }
}

impl<'a, L, T> ConcreteIterator<'a, L, T>
where
    L: List<T>
{
    pub fn new(l: &'a L) -> Self {
        Self {
            c_list: l,
            current_idx: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> List<T> for ConcreteList<T> {
    fn push(&mut self, t: T) {
        self.list.push(t);
    }

    fn pop(&mut self) -> T {
        self.list.pop().unwrap()
    }

    fn index_at(&self, i: usize) -> &T {
        &self.list[i]
    }

    fn length(&self) -> usize {
        self.list.len()
    }
}

impl<'a, L, T> Iterator<'a, T> for ConcreteIterator<'a, L, T>
where L: List<T>
{
    fn next(&mut self) -> Option<&'a T> {
        if self.current_idx < self.c_list.length() {
            let item = self.c_list.index_at(self.current_idx);
            self.current_idx += 1;
            Some(item)
        } else {
            None
        }
    }

    fn has_next(&self) -> bool {
        self.current_idx < self.c_list.length()
    }
}
