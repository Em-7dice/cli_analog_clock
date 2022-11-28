use std::{fs, f32::consts::TAU};
use chrono::prelude::*;
fn main() {
    //prove that time exists
    // let now_string = Local::now().time()
    // .format("%-I:%M:%S %p").to_string();
    // println!("The time is now {}", now_string);
        
    //set up canvas
    const X:usize = 35; //these are the hard-coded dimentions
    const Y:usize = 17; //of the clock face text file
    let mut canvas:[[char;Y];X] = [['#';Y];X];
    
    // set up clock face on canvas
    let clock_face_string = fs::read_to_string("src/ascii_clockface.txt")
        .expect("Failed to read file");
    let clock_face_string = clock_face_string.replace("\n", "");
    // println!("{}", clock_face_string);
    for y in 0..Y{
        for x in 0..X{
            canvas[x][y] = clock_face_string.chars()
                .nth(x + y * X)
                .expect("Error writing to canvas")
        }
    }
    
    //set the center of the canvas to '·'
    // '·' is extended ascii, which idk if it's a single bite or not.
    canvas[X/2][Y/2] = '·';
    
    //loop over every 1/24th of a circle
    //to check that my trig bullshit works
    // loop{
    //     let step = 1.0/120.0;
    //     let mut iter = step;
    //     while iter < 1.0{
    //         let coords = draw_radial(iter, X, Y, 1.0/3.0);
    //         let x = coords.first().expect("X Coordinate Error").clone();
    //         let y = coords.last().expect("Y Coordinate Error").clone();
    //         canvas[x][y] = '+';
    //         iter += step;
    //     }
    //     break
    // }
    
    //draw a point where the hour hand should be
    let mut hour_as_minutes = Local::now().time()
        .format("%I").to_string().parse::<usize>().expect("Error at hour_as_minutes")
        * 60;
    let minutes = Local::now().time()
        .format("%M").to_string().parse::<usize>().expect("Error at minutes");
    hour_as_minutes += minutes;
    let hour_fraction = hour_as_minutes as f32 / 720.0; // 720 minues in 12 hours
    let hour_point = draw_radial(hour_fraction, X, Y, 1.0/3.0);
    let x = hour_point.first().expect("X Coordinate Error").clone();
    let y = hour_point.last().expect("Y Coordinate Error").clone();
    canvas[x][y] = 'H';
    
    //draw a point where the minute hand should be
    let minute_fraction = minutes as f32 / 60.0;
    let minute_point = draw_radial(minute_fraction, X, Y, 2.0/3.0);
    let x = minute_point.first().expect("X Coordinate Error").clone();
    let y = minute_point.last().expect("Y Coordinate Error").clone();
    canvas[x][y] = 'M';
    
    //print the contents of the array
    for y in 0..Y{
       for x in 0..X{
           print!("{}", canvas[x][y]);
       }
       println!();
    }
}

fn lerp(input:f32, min_in:f32,max_in:f32, min_out:f32,max_out:f32) -> f32{
    let range_in = max_in - min_in;
    let range_out = max_out - min_out;
    
    let mut normal = input - min_in;
    normal *= 1.0/range_in;
    
    let mut output = normal * range_out;
    output += min_out;
    output
}

fn draw_radial(angle_as_fraction:f32, bounds_x:usize, bounds_y:usize, radius:f32) -> [usize;2]{ 
    //radius is a fraction, where 1 is the max of the bounds
    let theta = TAU * angle_as_fraction;
    
    let x = lerp(theta.sin(), 
        0.0, 1.0 / radius,
        bounds_x as f32/2.0, bounds_x as f32);
        
    let y = lerp(-1.0*theta.cos(), 
        0.0, 1.0 / radius, 
        bounds_y as f32/2.0, bounds_y as f32); 
    let x = x.floor() as usize;
    let y = y.floor() as usize;
    let coords = [x,y];
    coords
}