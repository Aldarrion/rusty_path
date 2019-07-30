use crate::vec3::{Vec3, sqr, random_in_unit_sphere, schlick};
use crate::ray::Ray;
use std::vec::Vec;
use std::boxed::Box;
use std::rc::Rc;
use rand::Rng;


pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}


pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}


pub struct Metal {
    albedo: Vec3,
    roughness: f32
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f32) -> Metal {
        Metal {
            albedo,
            roughness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit.p, ray.direction().normalized().reflect(&hit.normal) + self.roughness * random_in_unit_sphere());
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((
                scattered,
                self.albedo
            ))
        } else {
            None
        }
    }
}


pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            (-hit.normal, 
            self.ref_idx,
            self.ref_idx * ray.direction().dot(&hit.normal) / ray.direction().length())
        } else {
            (hit.normal, 
            1.0 / self.ref_idx,
            -ray.direction().dot(&hit.normal) / ray.direction().length())
        };

        let attuneation = Vec3::new(1.0, 1.0, 1.0);
        if let Some(refracted) = ray.direction().refract(&outward_normal, ni_over_nt) {
            if rand::thread_rng().gen::<f32>() >= schlick(cosine, self.ref_idx) {
                return Some((Ray::new(hit.p, refracted), attuneation));
            }
        }

        Some((Ray::new(hit.p, ray.direction().reflect(&hit.normal)), attuneation))
    }
}


pub struct HitRecord {
    pub material: Rc<Material>,
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    material: Rc<Material>,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<Material>) -> Sphere {
        Sphere {
            center,
            material,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(&oc) - sqr(self.radius);
        let discriminant = sqr(b) - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center) / self.radius,
                    t: temp
                });
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center) / self.radius,
                    t: temp
                });
            }
        }

        None
    }
}

pub struct HittableList {
    pub items: Vec<Box<Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest = t_max;
        for item in &self.items {
            let current = item.hit(r, t_min, closest);
            match &current {
                Some(current_hit) => {
                    closest = current_hit.t;
                    result = current;
                },
                _ => ()
            }
        }

        result
    }
}

