//#![allow(warnings)]
pub mod image;
pub mod ui;

#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

pub struct Rectangle<T> {
    pub position: Vector2<T>,
    pub size: Vector2<T>,
}

impl<T> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Rectangle<T> {
        let position = Vector2::new(x, y);
        let size = Vector2::new(width, height);
        Rectangle { position, size }
    }
}
