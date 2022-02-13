use generative_art::barium::{Color, LineEnd, Renderer, Shape, Vec2};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
    scale: f32,
    center_offset: f32,
    current_fill_color: Color,
    current_stroke_color: Color,
    current_line_width: f32,
    current_line_end: LineEnd,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl CanvasRenderer {
    pub fn new(id: &str, background: Option<Color>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();

        let canvas_size = Vec2::new(
            canvas
                .get_attribute("width")
                .unwrap()
                .parse::<f32>()
                .unwrap(),
            canvas
                .get_attribute("height")
                .unwrap()
                .parse::<f32>()
                .unwrap(),
        );

        let scale = canvas_size.x / 2.0;
        let center_offset = canvas_size.y / 2.0 / scale;

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        if let Some(background) = background {
            context.set_fill_style(&JsValue::from_str(background.as_hex(false).as_str()));
            context.set_global_alpha(background.a() as f64);
            context.fill_rect(0.0, 0.0, canvas_size.x as f64, canvas_size.y as f64);
        }

        context.set_fill_style(&JsValue::from_str(&Color::white().as_hex(false)));
        context.set_stroke_style(&JsValue::from_str(&Color::white().as_hex(false)));
        context.set_line_width(1.0);
        context.set_line_cap("butt");

        Self {
            context,
            scale,
            center_offset,
            current_fill_color: Color::white(),
            current_stroke_color: Color::white(),
            current_line_width: 1.0,
            current_line_end: LineEnd::Butt,
        }
    }
}

impl Renderer for CanvasRenderer {
    type Output = ();

    fn render(&mut self, shape: &Shape) {
        // Transform from Camera Space (range from (-1, -1) to (1, 1)) to Image Space (range from (0, 0) to (1000, 1000)).
        let mut points = shape.points.iter().map(|p| {
            let p = Vec2::new(p.x + 1.0, -p.y + self.center_offset);
            p * self.scale
        });

        if let Some(first) = points.next() {
            self.context.begin_path();

            self.context.move_to(first.x as f64, first.y as f64);

            for point in points {
                self.context.line_to(point.x as f64, point.y as f64);
            }

            // Fix ends of polygon
            if shape.is_polygon() {
                self.context.close_path();
            }

            if let Some(fill) = shape.fill {
                if self.current_fill_color != fill {
                    self.context
                        .set_fill_style(&JsValue::from_str(&fill.as_hex(true)));
                }

                self.context.fill();
            }

            if let Some(stroke) = shape.stroke {
                if self.current_stroke_color != stroke.color {
                    self.context
                        .set_stroke_style(&JsValue::from_str(&stroke.color.as_hex(true)));
                }

                if self.current_line_width != stroke.width {
                    self.context
                        .set_line_width(stroke.width as f64 * self.scale as f64);
                    self.current_line_width = stroke.width;
                }

                if self.current_line_end != stroke.line_end {
                    match stroke.line_end {
                        LineEnd::Butt => self.context.set_line_cap("butt"),
                        LineEnd::Round => self.context.set_line_cap("round"),
                    }
                }

                self.context.stroke();
            }
        }
    }

    fn finalize(self) -> Self::Output {}
}
