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

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Into<Hittable>) {
        self.objects.push(object.into());
    }
}

impl FromIterator<Hittable> for HittableList {
    fn from_iter<T: IntoIterator<Item = Hittable>>(iter: T) -> Self {
        Self {
            objects: iter.into_iter().collect(),
        }
    }
}
