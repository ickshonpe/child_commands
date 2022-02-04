// Not sure that I like this, or that it is needed.

use crate::ChildCommands;
use bevy::prelude::*;
use crate::*;

pub trait WithId {
    fn with_id(&mut self, f: impl FnMut(Entity)) -> &mut Self;
}

impl WithId for ChildCommands<'_, '_, '_> {
    fn with_id(&mut self, mut f: impl FnMut(Entity)) -> &mut Self {
        f(self.id());
        self
    }
}

pub fn usage(
    commands: &mut Commands
) -> Vec<Entity> {
    let mut out = vec![];
    let id = commands
    .spawn()
    .with_child()
    .with_id(|id| out.push(id))
    .with_child()
    .with_id(|id| out.push(id))
    .with_sibling()
    .with_id(|id| out.push(id))
    .id();
    out
}