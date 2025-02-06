// this is where we read the settings file from assets/settings.ron 
// and create the client or server app depending on the passed CLI arguments.
mod protocol;
use bevy::math::Vec2;
use protocol::*;

fn main(){
    let qwe = Vec2{x: 1, y: 2};
    let abc = PlayerPosition(qwe);
    print!("{abc}");
}