use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec: Option<HitRecord> = None;
        for obj in &self.objects {
            let (hit, record) = obj.hit(r, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                let hit_rec = record.unwrap();
                closest_so_far = hit_rec.t;
                temp_rec = Some(hit_rec);
            }
        }
        return (hit_anything, temp_rec);
    }
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    /// remove all the objects in the list
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    
    pub fn new()->Self{
        HittableList{objects:Vec::new()}
    }
}
