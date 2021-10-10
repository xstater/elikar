use xecs::EntityId;

/// A simple wrapper of SparseSet with a simple ID allocator
#[derive(Debug, Clone)]
pub struct SparseSet<T: Sized> {
    next_id: EntityId,
    sparse_set: xecs::sparse_set::SparseSet<EntityId, T>,
}

impl<T: Sized> SparseSet<T> {
    pub fn new() -> SparseSet<T> {
        SparseSet {
            next_id: unsafe { EntityId::new_unchecked(1) },
            sparse_set: xecs::sparse_set::SparseSet::new(),
        }
    }

    pub fn add(&mut self, data: T) -> EntityId {
        let id = self.next_id;
        self.next_id = unsafe { EntityId::new_unchecked(self.next_id.get() + 1) };
        self.sparse_set.add(id, data);
        id
    }

    pub fn empty(&self) -> bool {
        self.sparse_set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.sparse_set.len()
    }

    pub fn remove(&mut self, id: EntityId) -> Option<T> {
        self.sparse_set.remove(id)
    }

    pub fn has(&self, id: EntityId) -> bool {
        self.sparse_set.exist(id)
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.sparse_set.get(id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.sparse_set.get_mut(id)
    }

    pub fn data(&self) -> &[T] {
        self.sparse_set.data()
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        self.sparse_set.data_mut()
    }
}
