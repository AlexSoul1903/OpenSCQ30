use crate::theme;
use cosmic::{
    iced::{Color, Point, Rectangle, mouse, window},
    widget::{canvas, canvas::Action},
};
use std::time::{Duration, Instant};

const PADDING: f32 = 24.0;
const HIT_RADIUS: f32 = 18.0;

/// An EQ response editor. Values come from the device, not simulated audio.
pub struct EqualizerLine<Message> {
    min: i16,
    max: i16,
    values: Vec<i16>,
    on_change: Option<Box<dyn Fn(u8, i16) -> Message>>,
}

pub struct State {
    dragging: Option<usize>,
    hovered: Option<usize>,
    from: Vec<f32>,
    target: Vec<i16>,
    changed_at: Instant,
    hover_at: Instant,
}

impl Default for State {
    fn default() -> Self {
        Self {
            dragging: None,
            hovered: None,
            from: Vec::new(),
            target: Vec::new(),
            changed_at: Instant::now(),
            hover_at: Instant::now(),
        }
    }
}

impl<Message> EqualizerLine<Message> {
    pub fn view(&self) -> cosmic::Element<'_, Message> {
        cosmic::widget::canvas(self)
            .width(cosmic::iced::Length::Fill)
            .height(200)
            .into()
    }

    pub fn new(min: i16, max: i16, values: Vec<i16>) -> Self {
        Self {
            min,
            max,
            values,
            on_change: None,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(u8, i16) -> Message + 'static) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    fn displayed_values(&self, state: &State, now: Instant) -> Vec<f32> {
        if state.target.len() != self.values.len() || state.from.len() != self.values.len() {
            return self.values.iter().map(|&value| f32::from(value)).collect();
        }
        // Direct manipulation follows the pointer. Preset changes ease into place.
        let progress = if state.dragging.is_some() {
            1.0
        } else {
            theme::ease(
                now.duration_since(state.changed_at).as_secs_f32() * 1000.0 / theme::MOTION_MS,
            )
        };
        state
            .from
            .iter()
            .zip(&state.target)
            .map(|(from, &to)| from + (f32::from(to) - from) * progress)
            .collect()
    }

    fn points(&self, state: &State, bounds: Rectangle) -> Vec<Point> {
        band_points(
            &self.displayed_values(state, Instant::now()),
            self.min,
            self.max,
            bounds.width,
            bounds.height,
        )
    }

    fn hit(&self, state: &State, bounds: Rectangle, cursor: mouse::Cursor) -> Option<usize> {
        let position = cursor.position_in(bounds)?;
        self.points(state, bounds)
            .iter()
            .position(|point| point.distance(position) <= HIT_RADIUS)
    }
}

fn band_points(values: &[f32], min: i16, max: i16, width: f32, height: f32) -> Vec<Point> {
    let width = (width - PADDING * 2.0).max(0.0);
    let height = (height - PADDING * 2.0).max(0.0);
    let range = (f32::from(max) - f32::from(min)).max(1.0);
    values
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            let x = if values.len() == 1 {
                0.5
            } else {
                index as f32 / (values.len() - 1) as f32
            };
            let y = 1.0 - ((value - f32::from(min)) / range).clamp(0.0, 1.0);
            Point::new(PADDING + x * width, PADDING + y * height)
        })
        .collect()
}

fn value_at_y(y: f32, height: f32, min: i16, max: i16) -> i16 {
    if max <= min {
        return min;
    }
    let normalized = 1.0 - ((y - PADDING) / (height - PADDING * 2.0).max(1.0)).clamp(0.0, 1.0);
    (f32::from(min) + normalized * (f32::from(max) - f32::from(min))).round() as i16
}

impl<Message> canvas::Program<Message, cosmic::Theme, cosmic::Renderer> for EqualizerLine<Message> {
    type State = State;

    fn update(
        &self,
        state: &mut State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let now = Instant::now();
        if self.values != state.target {
            state.from = self.displayed_values(state, now);
            state.target = self.values.clone();
            state.changed_at = now;
        }
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if self.on_change.is_some() =>
            {
                state.dragging = self.hit(state, bounds, cursor);
                if state.dragging.is_some() {
                    return Some(Action::capture());
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.dragging.take().is_some() {
                    return Some(Action::request_redraw().and_capture());
                }
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let (Some(index), Some(change), Some(position)) =
                    (state.dragging, &self.on_change, cursor.position())
                {
                    return Some(
                        Action::publish(change(
                            index as u8,
                            value_at_y(position.y - bounds.y, bounds.height, self.min, self.max),
                        ))
                        .and_capture(),
                    );
                }
                let hovered = if self.on_change.is_some() {
                    self.hit(state, bounds, cursor)
                } else {
                    None
                };
                if hovered != state.hovered {
                    state.hovered = hovered;
                    state.hover_at = now;
                    return Some(Action::request_redraw());
                }
            }
            canvas::Event::Window(window::Event::Unfocused) => {
                state.dragging = None;
            }
            _ => (),
        }
        let duration = Duration::from_secs_f32(theme::MOTION_MS / 1000.0);
        if theme::motion_enabled()
            && (now.duration_since(state.changed_at) < duration
                || now.duration_since(state.hover_at) < duration)
        {
            Some(Action::request_redraw_at(now + Duration::from_millis(16)))
        } else {
            None
        }
    }

    fn draw(
        &self,
        state: &State,
        renderer: &cosmic::Renderer,
        theme: &cosmic::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<cosmic::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let studio = theme::is_studio(theme);
        let gold = if studio {
            theme::GOLD
        } else {
            theme.cosmic().accent_color().into()
        };
        let text = if studio {
            theme::TEXT
        } else {
            theme.cosmic().on_bg_color().into()
        };
        let grid = if studio {
            theme::SURFACE
        } else {
            theme.cosmic().primary(false).base.into()
        };
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            if studio {
                theme::SHELL
            } else {
                theme.cosmic().bg_color().into()
            },
        );
        let points = self.points(state, bounds);
        for step in 0..=4 {
            let y = PADDING + (bounds.height - PADDING * 2.0).max(0.0) * step as f32 / 4.0;
            frame.stroke(
                &canvas::Path::line(
                    Point::new(PADDING, y),
                    Point::new((bounds.width - PADDING).max(PADDING), y),
                ),
                canvas::Stroke::default().with_color(grid).with_width(1.0),
            );
        }
        for point in &points {
            frame.stroke(
                &canvas::Path::line(
                    Point::new(point.x, PADDING),
                    Point::new(point.x, (bounds.height - PADDING).max(PADDING)),
                ),
                canvas::Stroke::default().with_color(grid).with_width(1.0),
            );
        }
        if self.min < 0 && self.max > 0 {
            let y = band_points(&[0.0], self.min, self.max, bounds.width, bounds.height)[0].y;
            frame.stroke(
                &canvas::Path::line(
                    Point::new(PADDING, y),
                    Point::new((bounds.width - PADDING).max(PADDING), y),
                ),
                canvas::Stroke::default()
                    .with_color(Color { a: 0.35, ..text })
                    .with_width(1.0),
            );
        }
        if let Some(first) = points.first() {
            let path = canvas::Path::new(|path| {
                path.move_to(*first);
                for pair in points.windows(2) {
                    let mid = (pair[0].x + pair[1].x) / 2.0;
                    // Horizontal tangents do not overshoot the gain bounds.
                    path.bezier_curve_to(
                        Point::new(mid, pair[0].y),
                        Point::new(mid, pair[1].y),
                        pair[1],
                    );
                }
            });
            for (width, alpha) in [(11.0, 0.05), (7.0, 0.1), (2.5, 1.0)] {
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(Color { a: alpha, ..gold })
                        .with_width(width)
                        .with_line_cap(canvas::LineCap::Round),
                );
            }
        }
        for (index, point) in points.iter().enumerate() {
            let active = state.dragging == Some(index) || state.hovered == Some(index);
            if active {
                let progress = theme::ease(
                    Instant::now().duration_since(state.hover_at).as_secs_f32() * 1000.0
                        / theme::MOTION_MS,
                );
                frame.fill(
                    &canvas::Path::circle(*point, 9.0 + progress * 4.0),
                    Color { a: 0.18, ..gold },
                );
            }
            frame.fill(
                &canvas::Path::circle(*point, if active { 6.0 } else { 4.5 }),
                gold,
            );
            frame.fill(&canvas::Path::circle(*point, 2.0), text);
        }
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.dragging.is_some() {
            mouse::Interaction::Grabbing
        } else if self.on_change.is_some() && self.hit(state, bounds, cursor).is_some() {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drag_gain_clamps_to_device_bounds() {
        assert_eq!(value_at_y(-50.0, 200.0, -120, 120), 120);
        assert_eq!(value_at_y(250.0, 200.0, -120, 120), -120);
        assert_eq!(value_at_y(100.0, 200.0, -120, 120), 0);
    }
    #[test]
    fn handles_empty_single_and_fixed_ranges() {
        assert!(band_points(&[], -120, 120, 400.0, 200.0).is_empty());
        let points = band_points(&[0.0], 0, 0, 400.0, 200.0);
        assert_eq!(points[0].x, 200.0);
        assert!(points[0].y.is_finite());
        assert_eq!(value_at_y(10.0, 20.0, 5, 5), 5);
    }
    #[test]
    fn curve_positions_match_gain_at_chart_edges() {
        assert_eq!(
            band_points(&[120.0, 0.0, -120.0], -120, 120, 400.0, 200.0),
            [
                Point::new(24.0, 24.0),
                Point::new(200.0, 100.0),
                Point::new(376.0, 176.0)
            ]
        );
    }
}
