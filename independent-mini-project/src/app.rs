use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TiledPlugin::default())
            .insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, app_startup)
            .add_systems(Update, (app_update, greet_timer).chain());
    }
}

fn app_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Person, Name("Bob Tyson".to_string()),
    ));
    commands.spawn((
        Person, Name("Mike Tyson".to_string())
    ));

    // Load and display map
    let map_handle: Handle<TiledMapAsset> = asset_server.load("map.tmx");
    commands.spawn(TiledMap(map_handle));
}
fn app_update(mut _commands: Commands, _asset_server: Res<AssetServer>) {}

fn greet_timer(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hey {}", name.0);
        }
    }
}
