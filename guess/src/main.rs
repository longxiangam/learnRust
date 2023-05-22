use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::render::camera::CameraPlugin;

const CLEAR:Color = Color::rgb(0.1,0.0,0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
       .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(hello_world_system)
        .run();

    let a = abc;

}


fn test<T:PluginGroup>(a:T){

}

struct  abc;

impl PluginGroup for abc {
    fn build(self) -> PluginGroupBuilder {
        //todo!();
        println!("组件组1122");
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())

    }
}

fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
}

fn hello_world_system(mut commands:Commands) {
    println!("hello world");
}