use super::VectorCanvas;

pub trait VectorSketcher {
    /// Runs the sketcher. Allows to run function before each iteration of the sketcher, providing progress (ranges from 0.0 to 1.0).
    fn run<F: Fn(f32)>(&mut self, before_iter: F) -> &VectorCanvas;
}
