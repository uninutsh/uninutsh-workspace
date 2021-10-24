use uninutsh::Vector2;

pub struct Camera {
    position: Vector2<f64>,
}

impl Camera {
    pub fn new(x: f64, y: f64) -> Camera {
        let position = Vector2::new(x, y);
        Camera { position }
    }
}
