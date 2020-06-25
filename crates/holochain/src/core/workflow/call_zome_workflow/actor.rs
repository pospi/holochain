use crate::core::state::workspace::WorkspaceError;
use ghost_actor::{actor_builder, ghost_actor, GhostControlHandler, GhostHandler, GhostSender};
use holochain_state::env::EnvironmentWrite;

pub mod factory {
    use super::*;

    // TODO: no need to make the factory an actor. The reason being, it needs to
    // dynamically use the workspace actor's channel_factory, which is an async
    // operation and can't be handled by a sync GhostHandle

    ghost_actor! {
        pub actor Event<WorkspaceError> {
            fn transaction() -> GhostSender<workspace::Event>;
        }
    }

    pub struct Actor {
        env: EnvironmentWrite,
        channel_factory: actor_builder::GhostActorChannelFactory<workspace::Actor>,
    }

    impl GhostControlHandler for Actor {}
    impl GhostHandler<Event> for Actor {}

    impl EventHandler for Actor {
        fn handle_transaction(&mut self) -> EventHandlerResult<GhostSender<workspace::Event>> {
            self.channel_factory().create_channel()
            todo!()
        }
    }
}

pub mod workspace {
    use super::*;
    ghost_actor! {
        pub actor Event<WorkspaceError> {
            fn foo() -> ();
        }
    }

    pub struct Actor {}

    impl GhostControlHandler for Actor {}
    impl GhostHandler<Event> for Actor {}

    impl EventHandler for Actor {
        fn handle_foo(&mut self) -> EventHandlerResult<()> {
            todo!()
        }
    }
}
