use quicksilver::{
    geom::{Rectangle, Transform, Triangle, Vector},
    graphics::{Background, Color},
    input::Key,
    lifecycle::{run, Settings, State, Window},
    Result,
};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SKI_WIDTH: f32 = 10.0;
const SKI_LENGTH: f32 = 50.0;
const SKI_TIP_LEN: f32 = 20.0;
const STEERING_SPEED: f32 = 3.5;
const MAX_ANGLE: f32 = 75.0;

struct Screen {
    ski_across_offset: f32,
    direction: f32,
}

impl Screen {
    fn steer(&mut self, side: f32) {
        self.direction += STEERING_SPEED * side;
        if self.direction > MAX_ANGLE {
            self.direction = MAX_ANGLE;
        } else if self.direction < -MAX_ANGLE {
            self.direction = -MAX_ANGLE;
        }
    }
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            ski_across_offset: 0.0,
            direction: 0.0,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() {
            self.steer(1.0);
        }
        if window.keyboard()[Key::Left].is_down() {
            self.steer(-1.0);
        }
        Ok(())
    }

    // fn event(
    //     &mut self,
    //     _event: &quicksilver::lifecycle::Event,
    //     _window: &mut Window,
    // ) -> Result<()> {
    //     Ok(())
    // }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw_ex(
            &Rectangle::new(
                (
                    SCREEN_WIDTH / 2.0 + self.ski_across_offset - SKI_WIDTH / 2.0,
                    SCREEN_HEIGHT * 15.0 / 16.0 - SKI_LENGTH / 2.0,
                ),
                (SKI_WIDTH, SKI_LENGTH),
            ),
            Background::Col(Color::PURPLE),
            Transform::translate(Vector::new(0, -SKI_LENGTH / 2.0 - SKI_TIP_LEN))
                * Transform::rotate(self.direction)
                * Transform::translate(Vector::new(0, SKI_LENGTH / 2.0 + SKI_TIP_LEN)),
            0,
        );

        window.draw_ex(
            &Triangle::new(
                Vector::new(
                    SCREEN_WIDTH / 2.0 + self.ski_across_offset - SKI_WIDTH / 2.0,
                    SCREEN_HEIGHT * 15.0 / 16.0 - SKI_LENGTH / 2.0,
                ),
                Vector::new(
                    SCREEN_WIDTH / 2.0 + self.ski_across_offset + SKI_WIDTH / 2.0,
                    SCREEN_HEIGHT * 15.0 / 16.0 - SKI_LENGTH / 2.0,
                ),
                Vector::new(
                    SCREEN_WIDTH / 2.0 + self.ski_across_offset,
                    SCREEN_HEIGHT * 15.0 / 16.0 - SKI_LENGTH / 2.0,
                ),
            ),
            Background::Col(Color::INDIGO),
            Transform::translate(Vector::new(0, -SKI_TIP_LEN * 2.0 / 3.0))
                * Transform::rotate(self.direction)
                * Transform::translate(Vector::new(0, SKI_TIP_LEN * 2.0 / 3.0)),
            0,
        );

        Ok(())
    }

    // fn handle_error(error: quicksilver::Error) {
    //     #[cfg(target_arch = "wasm32")]
    //     {
    //         let message = format!("Unhandled error: {:?}", error);
    //         console!(error, message);
    //     }
    //     panic!("Unhandled error: {:?}", error);
    // }
}

fn main() {
    run::<Screen>(
        "Ski",
        Vector::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        Settings {
            draw_rate: 40.0,
            update_rate: 40.0,
            ..Settings::default()
        },
    );
}
