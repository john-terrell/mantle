pub struct Camera {
    o: Ray,                         // View ray
    up: Vector,                     // View up vector
    right: Vector,                  // View right vector
    aspectRatio: f64,               // View aspect ratio
    fieldOfView: Angle<Degrees>,    // Horizontal field of view
}
