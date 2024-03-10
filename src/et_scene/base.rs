use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;


pub struct ETScene<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> ETScene<'w, 's> {
    pub fn new(commands: Commands<'w, 's>) -> ETScene<'w, 's> {
        ETScene {
            commands
        }
    }

    pub fn add_entity<'b, T>(mut self, bundle: T) -> ETScene<'w, 's>
        where
            T: Bundle + 'w,
            'b: 'w,
    {
        println!("Add new entity properly");
        let entity = self.commands.spawn(bundle);
        // Return self cause self isn't a pointer in this case
        return self;
    }
}
