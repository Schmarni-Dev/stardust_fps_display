use serde::{Deserialize, Serialize};
use stardust_xr_asteroids::{
    ClientState, Context, CustomElement, Entity, Migrate, Reify, Tasker, Transformable,
    components::Grabbable, elements::Text,
};
use stardust_xr_fusion::{client::FrameInfo, fields::Shape, types::Posef};

#[tokio::main]
async fn main() {
    stardust_xr_asteroids::client::run::<State>(&[])
        .await
        .unwrap();
}

fn a<const N: usize>() -> [f32; N] {
    [0.0; N]
}

#[derive(Serialize, Deserialize)]
struct State {
    #[serde(skip, default = "a")]
    frame_time: [f32; 120],
    pose: Posef,
}
impl Default for State {
    fn default() -> Self {
        Self {
            frame_time: [0.0; _],
            pose: Posef::default(),
        }
    }
}
impl ClientState for State {
    const APP_ID: &'static str = "dev.schmarni.stardust.fps";
    fn on_frame(&mut self, info: &FrameInfo) {
        self.frame_time.rotate_left(1);
        self.frame_time[self.frame_time.len() - 1] = info.delta;
    }
}
impl Migrate for State {
    type Old = State;
}
impl Reify for State {
    fn reify(
        &self,
        _context: &Context,
        _tasks: impl Tasker<Self>,
        _: (),
    ) -> impl stardust_xr_asteroids::Element<Self> {
        let delta = self.frame_time.iter().fold(0.0, |a, b| a + b) / self.frame_time.len() as f32;
        Entity::new(Shape::Sphere { radius: 0.05 })
            .component(Grabbable::new(|state: &mut State, pose| state.pose = pose))
            .pose(self.pose)
            .build()
            .child(Text::new(format!("{}ms\nfps: {}", delta * 1000.0, 1.0 / delta)).build())
    }
}
