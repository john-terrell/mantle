use geometry::vector::*;
use geometry::ray::*;
use geometry::angle::*;

pub struct Camera {
    n: Ray,             // View ray
    up: Vector,         // View up vector
    right: Vector,      // View right vector
    aspect_ratio: f64,   // View aspect ratio
    field_of_view: Angle, // Horizontal field of view
}

impl Camera {
    pub fn get_camera_vector(&self, s: f64, t: f64) -> Ray {
        // get the center of the plant
        let distance = (self.aspect_ratio / 2.0) / (self.field_of_view.in_radians() / 2.0).tan();

        let image_plane_center = self.n.parameterize(distance);

        let left_ray = Ray{ 
            o: image_plane_center,
            d: self.right.inverse().of_magnitude(self.aspect_ratio / 2.0),
        };

        let up_ray = Ray{
            o: left_ray.parameterize(1.0),
            d: self.up.normalized(),
        };

        let right_ray = Ray{
            o: up_ray.parameterize(0.5),
            d: self.right.of_magnitude(self.aspect_ratio),
        };

        let down_ray = Ray{
            o: right_ray.parameterize(s),
            d: self.up.inverse().normalized(),
        };

        let pixel_location = down_ray.parameterize(t);
        let eye_normal = Vector::subtract(pixel_location, self.n.o);

        return Ray{ 
            o: self.n.o,
            d: eye_normal,
        };
    }
}
