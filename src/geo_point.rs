// Struct of point. In the implementation for DB here will be longitude (x), latitude (y)

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
}