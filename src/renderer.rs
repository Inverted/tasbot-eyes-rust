use crate::gif::Animation;

pub trait Renderer{
    fn play(&self, anim: &Animation);
}