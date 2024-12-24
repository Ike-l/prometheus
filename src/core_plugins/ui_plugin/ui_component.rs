use super::{
    acceleration_structures_plugin::{
        prelude::Collider, 
        AccelerationStructure
    }, 
    label_plugin::prelude::LabeledEntities, 
    ui_acceleration_structure::UIAccelerationStructure, 
    CursorPosition, DeviceEventBus, EventReader, MutWorld, Res, ResMut, WindowEventBus
};

#[derive(Debug, Clone)]
pub enum Event {
    Window(WindowEventBus),
    Device(DeviceEventBus)
}

#[derive(Debug, Default)]
pub struct UIComponent {
    pub event_buffer: Vec<Event>,
}

pub fn input(
    window_events: EventReader<WindowEventBus>, 
    device_events: EventReader<DeviceEventBus>, 
    mut world: MutWorld,
    acc_struct: Res<UIAccelerationStructure>,
    mut cursor_position: ResMut<CursorPosition>,
    labels: Res<LabeledEntities>
) {
    for event in window_events.read() {
        match event.0 {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                cursor_position.x = position.x;
                cursor_position.y = position.y;
            }
            _ => {
                log::info!("Window Event captured in UI: {event:?}");
                let collisions = acc_struct.query(&cursor_position);
                handle_event(
                    &collisions,
                    &labels, 
                    &Event::Window(event.clone()),
                    &mut world
                );
            }
        }
    }

    let collisions = acc_struct.query(&cursor_position);
    for event in device_events.read() {
        log::info!("Device Event captured in UI: {event:?}");
        handle_event(
            &collisions, 
            &labels, 
            &Event::Device(event.clone()),
            &mut world
        );
    }
}

fn handle_event(
    collisions: &Vec<Collider>, 
    labels: &LabeledEntities, 
    event: &Event,
    world: &mut MutWorld,
) {
    for collision in collisions {
        let entity = labels.get_entity(&collision.entity_label).expect("Collided Entity has invalid label");
        match world.query_one_mut::<&mut UIComponent>(*entity) {
            Ok(component) => {
                component.event_buffer.push(event.clone());
            },
            Err(_) => {
                unreachable!("Accleration Structure has an entity without a `UIComponent`")
            }
        } 
    }
}

// #[derive(Debug)]
// pub enum Delay {
//     Time(Duration),
//     Tick(Tick),
// }

// impl Default for Delay {
//     fn default() -> Self {
//         Self::Tick(Tick(1))
//     }
// }


// #[derive(Debug, Default)]
// pub struct DelayedUIComponent {
//     pub event: Option<Event>,
//     delay_progress: Delay,
//     delay_target: Delay,
// }


