<div align="left">
<a href="https://github.com/BlackPhlox/bevy_config_cam"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/config_cam.svg" width="970" height="200" alt="bevy config cam"></a>
</div>

[![crates.io](https://img.shields.io/crates/v/bevy_config_cam)](https://crates.io/crates/bevy_config_cam)</br>[![docs.rs](https://docs.rs/bevy_config_cam/badge.svg)](https://docs.rs/bevy_config_cam)

An easy plug-n-play multifunctional camera that allows for easy setup of a camera and player for a scene.

Add a few lines of code to your existing project allows you to test a wide variety of cameras by attaching it to
your player asset or the default red player entity. 

Inspired and extended on [bevy_flycam](https://github.com/sburris0/bevy_flycam), this plugin is fairly simple but should be a neat help for developers looking for a camera implementation without going the hassle of reinventing the wheel.

# Showcase

## Camera Modes

LookAt | FollowStatic
:-------------------------:|:-------------------------:
<img src="https://user-images.githubusercontent.com/25123512/119991088-5fb59700-bfc9-11eb-88d1-44a2b2a47cfa.png" alt="LookAt">  |  <img src="https://user-images.githubusercontent.com/25123512/119991121-6cd28600-bfc9-11eb-9fcd-89fa9d591d4b.png" alt="FollowStatic">

TopDown | TopDownDirection
:-------------------------:|:-------------------------:
<img src="https://user-images.githubusercontent.com/25123512/119991141-71973a00-bfc9-11eb-90ee-377ffe656ae2.png" alt="TopDown"> |  <img src="https://user-images.githubusercontent.com/25123512/119991151-7360fd80-bfc9-11eb-9d41-1947c997b98a.png" alt="TopDownDirection">

FollowBehind | Fps
:-------------------------:|:-------------------------:
<img src="https://user-images.githubusercontent.com/25123512/119991175-7956de80-bfc9-11eb-901c-0164152dd97d.png" alt="FollowBehind">  |  <img src="https://user-images.githubusercontent.com/25123512/119991187-7bb93880-bfc9-11eb-9739-598e01ba214f.png" alt="Fps">

Free | ?
:-------------------------:|:-------------------------:
<img src="https://user-images.githubusercontent.com/25123512/119991199-7fe55600-bfc9-11eb-9ae6-b29cec3a3cd7.png" alt="Free" >  |  Any suggestions on other camera modes you want? Let me know by creating an issue :)

## Settings

| Camera Mode | Disc. | Demo |
| --- | --- | --- |
| MovementSpeed (Camera) | Change the speed of there cameras movement | |
| Sensitivity | Change the sensitivity of the cameras mouse movement | |
| Lerp | Change the linera interpolation between the target and the player (LookAt cameramode only)||
| Zoom/FOV| Change the FOV of the camera | <img src="https://user-images.githubusercontent.com/25123512/119991256-8d9adb80-bfc9-11eb-9e1d-5d763ec150a2.png" alt="LowFOV" width="250"> |
| CamFwd (Unused) | Toggle between xyz movement relative to world coords or camera local coords and rotation (Free cameramode only)||

# Getting started

## Test the project

Before I start using a plugin, I all ways like to run the examples prior to setting it up for my own project. Just to see if the plugin satisfy my needs. 
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

Adding this plugin to your project is *very* simple, it only requires you to write 2 lines of code. 

### Step 1. - Setup
    
Add the correct version to your `Cargo.toml`, you can find the version you looking for under the support section. The thing you should be adding should look like this (only add the line marked by `# <--`):
```toml
[dependencies]
bevy = { version = "0.5"}
# ...
bevy_config_cam = { version = "0.1.1"} # <-- 
```

### Step 2. - Add to project

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

### Step 3. - Config (Optional)

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
        player_asset: "models/craft_speederA.glb#Scene0", //Model of the player, default is a red cube
        ..Default::default()
    })
```
Note: That some of them are overwritten by accessing the settings or the changing the camera type. Feedback on this is high appreciated, just create a new issue and I'll look into it when I have the time.

# Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_multicam|
|---|---|
|0.5|0.1.X|

# Licensing
The project is under dual license MIT and ISC (functionally equivalent, though ISC removing some language that is no longer necessary), so joink to your hearts content, just remember the license agreements.

# Contributing
Yes this project is still very much WIP, so PRs are very welcome
