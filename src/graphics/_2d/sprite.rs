use std::marker::PhantomData;
use cgmath::Point3;
use cgmath::prelude::*;

//marker structs later

struct Sprite<T> {
    position: Point3<f32>,
    _marker: PhantomData<T>,
}
