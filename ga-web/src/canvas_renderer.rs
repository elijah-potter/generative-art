use generative_art::denim::{Color, Renderer, Shape, Vec2, LineEnd};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

pub struct CanvasRendererSettings {
    pub id: String,
    pub background: Option<Color>,
}

pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
    scale: f32,
    center_offset: f32,
}

impl Renderer for CanvasRenderer {
    type Settings = CanvasRendererSettings;

    type Output = ();

    fn new(settings: Self::Settings) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&settings.id).unwrap();

        let canvas_size = Vec2::new(canvas
            .get_attribute("width")
            .unwrap()
            .parse::<f32>().unwrap(), canvas
            .get_attribute("height")
            .unwrap()
            .parse::<f32>().unwrap());

        let scale = 
            canvas_size.x
            / 2.0;
        let center_offset = canvas_size.y
            / 2.0
            / scale;

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

        if let Some(background) = settings.background{
            context.set_fill_style(&JsValue::from_str(background.as_hex(false).as_str()));
            context.set_global_alpha(background.a() as f64);
            context.fill_rect(0.0, 0.0, canvas_size.x as f64, canvas_size.y as f64);
        }

        Self {
            context,
            scale,
            center_offset,
        }
    }

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
                self.context
                    .set_fill_style(&JsValue::from_str(&fill.as_hex(false)));
                self.context.set_global_alpha(fill.a() as f64);

                self.context.fill();
            }

            if let Some(stroke) = shape.stroke {
                self.context
                    .set_stroke_style(&JsValue::from_str(&stroke.color.as_hex(false)));
                self.context.set_global_alpha(stroke.color.a() as f64);
                self.context.set_line_width(stroke.width as f64 * self.scale as f64);

                match stroke.line_end{
                    LineEnd::Butt => self.context.set_line_cap("butt"),
                    LineEnd::Round => self.context.set_line_cap("round"),
                }

                self.context.stroke();
            }
        }
    }

    fn finalize(self) -> Self::Output {}
}
