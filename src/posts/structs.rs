#[derive(Debug)]
pub struct List<T> {
    items: Vec<T>,
}

#[derive(Debug)]
pub struct ItemRef<'a, T> {
    index: usize,
    list: &'a List<T>,
}

#[derive(Debug)]
pub struct Neighbors<'a, T> {
    pub item: &'a T,
    pub prev: Option<&'a T>,
    pub next: Option<&'a T>,
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = ItemRef<'a, T>;
    type IntoIter = std::vec::IntoIter<ItemRef<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter()
            .enumerate()
            .map(|(index, _)| ItemRef { list: &self, index })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T> List<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

impl<'a, T> ItemRef<'a, T> {
    pub fn get(&self) -> &T {
        self.list.items.get(self.index).expect("invalid reference")
    }
    pub fn prev(&self) -> Option<&T> {
        let index = self.index.checked_sub(1)?;
        self.list.items.get(index)
    }
    pub fn next(&self) -> Option<&T> {
        let index = self.index + 1;
        self.list.items.get(index)
    }
    pub fn neighbors(&self) -> Neighbors<T> {
        let item = self.get();
        let prev = self.prev();
        let next = self.next();
        Neighbors { item, prev, next }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
