pub type Point3i64 = (i64, i64, i64);

pub fn distance_3d(p1: Point3i64, p2: Point3i64) -> i64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let dz = p2.2 - p1.2;
    (dx * dx + dy * dy + dz * dz).isqrt()
}
