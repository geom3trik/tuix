use crate::{Command, Entity, Length, Property};
use flume::Sender;

#[derive(Clone)]
pub struct Handle {
    pub entity: Entity,
    pub(crate) command_sender: Sender<Command>,
}

impl Handle {
    pub fn new(entity: Entity, command_sender: Sender<Command>) -> Self {
        Self {
            entity,
            command_sender,
        }
    }
}

impl Handle {
    pub fn set_width(&self, value: Length) -> &Handle {
        self.command_sender
            .try_send(Command::SetProperty(self.entity, Property::Width(value)));
        self
    }

    pub fn set_height(&self, value: Length) -> &Handle {
        self.command_sender
            .try_send(Command::SetProperty(self.entity, Property::Height(value)));
        self
    }
}
