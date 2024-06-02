use super::Hittable;

pub struct HittableList {
    pub(super) objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
}

impl FromIterator<Hittable> for HittableList {
    fn from_iter<T: IntoIterator<Item = Hittable>>(iter: T) -> Self {
        Self {
            objects: iter.into_iter().collect(),
        }
    }
}
