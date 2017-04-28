use engine::gameobjects::GameObject;

struct AnimationComponent;
struct PlayerComponent;

impl ComponentUpdate for AnimationComponent
{
    fn Update(&self, game_obj : &GameObject , time : f32){
        ()
    }
}

pub trait ComponentUpdate {
    fn Update(&self, game_obj : &GameObject , time : f32);
    // add code here
}