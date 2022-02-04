use bevy::ecs::system::*;
use bevy::prelude::*;

pub struct ChildCommands<'w, 's, 'a> {
    parent: Entity,
    entity: Entity,
    entity_commands: &'a mut EntityCommands<'w, 's, 'a>,
}

pub trait SpawnChild<'w, 's, 'a> {
    fn with_child(&'a mut self) -> ChildCommands<'w, 's, 'a>;
    fn with_child_bundle(&'a mut self, bundle: impl Bundle) -> ChildCommands<'w, 's, 'a> {
        let mut child_commands = self.with_child();
        child_commands.insert_bundle(bundle);
        child_commands
    }
}

impl<'w, 's, 'a> SpawnChild<'w, 's, 'a>  for EntityCommands<'w, 's, 'a> {
    fn with_child(&'a mut self) -> ChildCommands<'w, 's, 'a> {
        let parent = self.id();
        let child = self.commands().spawn().id();
        self.commands().add(AddChild { child, parent });
        ChildCommands { 
            parent,
            entity: child,
            entity_commands: self
        }
    } 
}

impl<'w, 's, 'a> SpawnChild<'w, 's, 'a> for ChildCommands<'w, 's, 'a> {
    fn with_child(&'a mut self) -> ChildCommands<'w, 's, 'a> {
        let parent = self.entity;
        let child = self.commands().spawn().id();
        self.commands().add(AddChild { child, parent });
        ChildCommands {
            parent,
            entity: child,
            entity_commands: self.entity_commands
        }
    }
}

impl <'w, 's, 'a> ChildCommands<'w, 's, 'a> {
    #[inline]
    #[must_use = "Omit the .id() call if you do not need to store the `Entity` identifier."]
    pub fn id(&self) -> Entity {
        self.entity
    }

    pub fn insert_bundle(&mut self, bundle: impl Bundle) -> &mut Self {
        let entity = self.id();
        self.commands().add(InsertBundle {
            entity,
            bundle,
        });
        self
    }

    pub fn insert(&mut self, component: impl Component) -> &mut Self {
        let entity = self.id();
        self.commands().add(Insert {
            entity,
            component,
        });
        self
    }

    pub fn with_sibling(&'a mut self) -> Self {
        let sibling = self.commands().spawn().id();
        let parent = self.parent;
        self.commands().add(AddChild { child: sibling, parent });
        ChildCommands { 
            parent,
            entity: sibling,
            entity_commands: self.entity_commands
        }
    }

    pub fn with_sibling_bundle<T: Bundle>(&'a mut self, bundle: T) -> Self {
        let mut child_commands = self.with_sibling();
        child_commands.insert_bundle(bundle);
        child_commands
    }

    pub fn with_children(&mut self, spawn_children: impl FnOnce(&mut ChildBuilder)) -> &mut EntityCommands<'w, 's, 'a> {
        let entity = self.entity;
        unsafe {
            let p = self.entity_commands as * mut EntityCommands;
            let q = p.clone();
            let ec = (*q).commands().entity(entity);
            let r = (&ec) as * const EntityCommands;
            std::ptr::swap(p, r as * mut EntityCommands);
        }
        self.entity_commands.with_children(spawn_children)
    }

    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        self.entity_commands.commands()
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Component)]
    struct A;

    #[derive(Component)]
    struct B;

    #[derive(Component)]
    struct C;

    #[derive(Component)]
    struct D;

    #[derive(Component)]
    struct E;

    fn spawn_hierachy_1(
        mut commands: Commands
    ) {
        commands
        .spawn()
        .insert(A)
        .with_child()
        .insert(B);
    }

    fn spawn_hierachy_2(
        mut commands: Commands
    ) {
        commands
        .spawn_bundle((A,))
        .with_child_bundle((B,));
    }

    fn spawn_hierachy_3(
        mut commands: Commands
    ) {
        commands
        .spawn_bundle((A,))
        .with_child_bundle((A, B,))
        .with_child_bundle((A, B, C,));
    }

    fn spawn_hierachy_4(
        mut commands: Commands
    ) {
        commands
        .spawn_bundle((A,))
        .with_child_bundle((A, B,))
        .with_child_bundle((A, B, C,))
        .with_sibling_bundle((A, B, C,))
        .with_sibling_bundle((A, B, C,));
    }

    fn spawn_hierachy_5(
        mut commands: Commands
    ) {
        let parent = commands
        .spawn_bundle((A,))
        .id();

        commands.entity(parent)
        .with_child_bundle((B,))
        .with_child_bundle((C,))
        .with_sibling_bundle((C,));
        
        commands.entity(parent)
        .with_child_bundle((B,))
        .with_child_bundle((C,))
        .with_sibling_bundle((C,));    
    }

    fn spawn_hierachy_6(
        mut commands: Commands
    ) {
        commands
        .spawn_bundle((A,))     
        
        .with_children(|builder| {
            builder
            .spawn_bundle((B,))
            .with_child_bundle((C,))
            .with_child_bundle((D,));

            builder
            .spawn_bundle((B,))
            .with_child_bundle((C,))
            .with_child_bundle((D,));
        });
    }

    #[test]
    fn spawn_child_1() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_1).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 2);
        let mut a = world.query::<(&Children, &A)>();
        let mut b = world.query::<(&Parent, &B)>();
        assert_eq!(a.iter(&world).len(), 1);
        assert_eq!(b.iter(&world).len(), 1);
        for (children, _) in a.iter(&world) {
            for child in children.iter() {
         
                assert!(b.get(&world, *child).is_ok());
            }
        }
    }

    #[test]
    fn spawn_child_2() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_2).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 2);
        let mut a = world.query::<(&Children, &A)>();
        let mut b = world.query::<(&Parent, &B)>();
        assert_eq!(a.iter(&world).len(), 1);
        assert_eq!(b.iter(&world).len(), 1);
        for (children, _) in a.iter(&world) {
            for child in children.iter() {
                assert!(b.get(&world, *child).is_ok());
            }
        }
    }

    #[test]
    fn spawn_child_3() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_3).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 3);
        assert_eq!(world.query::<(&Children, &A)>().iter(&world).len(), 2);
        assert_eq!(world.query::<(&Parent, &Children, &A, &B)>().iter(&world).len(), 1);
        assert_eq!(world.query::<(&Parent, &A, &B, &C)>().iter(&world).len(), 1);
    }

    #[test]
    fn spawn_child_4() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_4).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 5);
        assert_eq!(world.query::<(&Children, &A)>().iter(&world).len(), 2);
        assert_eq!(world.query::<(&Parent, &Children, &A, &B)>().iter(&world).len(), 1);
        assert_eq!(world.query::<(&Parent, &A, &B, &C)>().iter(&world).len(), 3);
    }

    #[test]
    fn spawn_child_5() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_5).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 7);
        assert_eq!(world.query::<(&Parent, &C)>().iter(&world).len(), 4);
    }

    #[test]
    fn spawn_child_6() {
        let mut world = World::default();
        SystemStage::single_threaded().add_system(spawn_hierachy_6).run(&mut world);
        assert_eq!(world.query::<Entity>().iter(&world).len(), 7);
        assert_eq!(world.query::<(&Parent, &D)>().iter(&world).len(), 2);
    }
}