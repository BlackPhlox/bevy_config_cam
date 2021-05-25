# bevy_config_cam

An easy plug-n-play multifunctional camera that allows for easy setup of a camera for a scene.

Add a few lines of code to your existing project allows you to test a wide variety of cameras by attaching it to
your player entity or the default spaceship entity. 

Inspired and extended on [bevy_flycam](https://github.com/sburris0/bevy_flycam), this plugin is fairly simple but should be a neat help for developers looking for a camera implementation without going the hassle of reinventing the wheel.

# Getting started

## Test the project

Before I using a plugin, I all ways like to run the examples prior to setting it up for my own project. Just to see if the plugin satisfy my needs. 
If you're like me then this is for you.

1. Clone this repo to your local machine
2. Go into the project folder (`cd bevy_config_cam`)
3. And run the cargo command (`cargo run --release --example simple`)
4. Test functionality </br>
Player : <kbd>↑</kbd><kbd>←</kbd><kbd>↓</kbd><kbd>→</kbd> for movement, <kbd>RShift</kbd> & <kbd>-</kbd> for going up and down.<br>
Camera : <kbd>W</kbd><kbd>A</kbd><kbd>S</kbd><kbd>D</kbd> for movement, <kbd>Space</kbd> & <kbd>LShift</kbd> for going up and down.<br>
Switch Camera: <kbd>C</kbd> (Look in console for which camera type you are on)</br>
Settings: <kbd>E</kbd> and use the mouse-scroll to change the selected settings value.

## Add to your own project

Adding this plugin is *fairly* simple. 

### Step 1. - Setup
    
Add the correct version to your `Cargo.toml`, you can find the version you looking for under the support section. The thing you should be adding should look like this (only add the line marked by `# <--`):
```
[dependencies]
bevy = { version = "0.5"}
# ...
bevy_config_cam = { version = "0.1"} # <-- 
```

### Step 2. - Install
And then run the cargo command `cargo install` to install the plugin.

### Step 3. - Add to project

```rust
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam) // <--
        // ... 
        // Your other plugins and game code here
        // ...
        .run();
}
```

### Step 4. - Config (Optional)

Now, there is a reason for the name `bevy_config_cam`. It is most likely that you want something more than just the default behavior. You might want to toggle between 2 types of cameras or allow the user to change the fov using a slider. Currently I haven't gotten far with creating a user-friendly api you can access but is something I will look into. For now you can insert two recourses to override the default behavior of the plugin, as seen in the example:

```rust
    .insert_resource(MovementSettings {
        sensitivity: 0.00015, // default: 0.00012
        speed: 15.0,          // default: 12.0
        dist: 5.0,            // Camera distance from the player in topdown view
        ..Default::default()
    })
    .insert_resource(PlayerSettings {
        pos: Vec3::new(2., 0., 0.),//Initial position of the player
        ..Default::default()
    })
```
Note: That some of them are overwritten by accessing the settings or the changing the camera type. Feedback on this is high appreciated, just create a new issue and I'll look into it when I have the time.

# Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_multicam|
|---|---|
|0.5|0.1|

# Licensing
The project is under dual license MIT and ISC (functionally equivalent, though ISC removing some language that is no longer necessary), so joink to your hearts content, just remember the license agreements.

# Contributing
Yes this project is still very much WIP, so PRs are very welcome
