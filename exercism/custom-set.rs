use std::collections::BTreeSet;

#[derive(PartialEq, Debug)]
pub struct CustomSet<T> {
    set: BTreeSet<T>
}

impl<T: Ord> CustomSet<T> {
    pub fn new(v: Vec<T>) -> Self {
        CustomSet{set: v.into_iter().collect()}
    }
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
    pub fn add(&mut self, t: T) -> bool {
        self.set.insert(t)
    }
    pub fn contains(&self, t: &T) -> bool {
        self.set.contains(t)
    }
    pub fn is_subset(&self, another: &Self) -> bool {
        self.set.is_subset(&another.set)
    }
    pub fn is_disjoint(&self, another: &Self) -> bool {
        self.set.is_disjoint(&another.set)
    }
}

impl<T: Ord+Clone> CustomSet<T> {
    pub fn intersection(&self, another: &Self) -> Self {
        Self::new(self.set.intersection(&another.set).cloned().collect())
    }
    pub fn difference(&self, another: &Self) -> Self {
        Self::new(self.set.difference(&another.set).cloned().collect())
    }
    pub fn union(&self, another: &Self) -> Self {
        Self::new(self.set.union(&another.set).cloned().collect())
    }
}
