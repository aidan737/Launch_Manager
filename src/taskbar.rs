extern crate piston_window;
use crate::Taskbar::Draw_func::Point2d;
pub use piston_window::*;
use serialport::{SerialPort, SerialPortBuilder};
use std::io::{self, Read};
use std::sync::{Arc, Mutex};


#[path = "textures/macropad.rs"] mod Macropad_sprite;
#[path = "textures/Draw_func.rs"] mod Draw_func;
#[path = "maths/button.rs"] mod Button;

#[path = "Devices/Macropad.rs"] mod Macropad;

pub struct object_information
{
     pub name: String,
     pub comport: String,

}


use serialport::SerialPortInfo;

static OBJECTS: Mutex<Vec<object_information>> = Mutex::new(vec![]);

static OBJECT_ON_SCREEN: Mutex<usize> = Mutex::new(0);
static FIRST_RUN: Mutex<bool> = Mutex::new(true);
static FIRST_RUN1: Mutex<bool> = Mutex::new(true);
static PORTS: Mutex<Vec<SerialPortInfo>> = Mutex::new(vec![]);

pub fn render(c: &Context, g: &mut G2d,mouse_position:[f64; 2], button_down:bool,keypressed:Key, Window_minimised:bool,scroll_delta:f64) -> usize
{
    //drawing the black bar
    rectangle([0.2,0.2, 0.2, 1.0], // red
              [0.0, 615.0, 1000.0, 75.0],
              c.transform, g);

    let mut objects_selected = OBJECT_ON_SCREEN.lock().unwrap();
let mut first = FIRST_RUN.lock().unwrap();
if(*first){
    let filename = "Launch_White.bmp"; // Replace with the actual file path
    display_image(filename,&c,g);
*first = false;
}

let mut first_run = FIRST_RUN1.lock().unwrap();

if(Window_minimised){
*first_run = true;
return 1;
}
    // auto update if ports change
    let ports = serialport::available_ports().expect("No ports found!");
    let mut old_port = PORTS.lock().unwrap();
    if(*old_port != ports)
    {

        cheack_coms();
        *old_port = ports;
        *objects_selected = 0;
        *first_run = true;
        return 1;
    }


if(*first_run){

    let mut pixels = PIXELS.lock().unwrap();
for pixel in pixels.iter()
{

    draw_pixel(pixel.x + 350.0, pixel.y+ 250.0, pixel.r, pixel.g, pixel.b, &c, g);

}
*first_run = false;
}






    //drawing all buttons for connected devices
    let mut objects = OBJECTS.lock().unwrap();

    for (index,object) in (&*objects).iter().enumerate() {
        if(object.name == "macropad"){
    Macropad_sprite::draw_Macropad_icon(&c,g,index);
            }
       if(Button::is_point_in_rectangle(&Point2d{point_x:mouse_position[0],point_y:mouse_position[1]},&Point2d{point_x: (80.0*index as f64), point_y:625.0},&Point2d{point_x: 60.0, point_y:60.0}))
       {
            if(button_down)
            {

                *objects_selected = index+1;
            }

       }


    }



    if(*objects_selected != 0)
    {
        if(objects[*objects_selected -1].name == "macropad".to_string())
        {
            Macropad::render_macropad(&c,g,mouse_position,button_down,&objects[*objects_selected -1].comport,keypressed,scroll_delta);

        }
    }

return *objects_selected;

}
use std::thread::sleep;
use std::time::Duration;
pub fn cheack_coms()
{
    let mut objects = OBJECTS.lock().unwrap();
    objects.clear();
    let ports = serialport::available_ports().expect("No ports found!");

        for port_info in ports {
            let port_name = port_info.port_name;
            println!("Trying port: {}", port_name);

            let mut port = serialport::new(&port_name, 9600).timeout(Duration::from_millis(500)).open().expect("Failed to open port");
             port.write_data_terminal_ready(true);

            // Create a buffer to store incoming data
            let mut buffer = [0u8; 1024];

            let mut found_macropad = false;

            // Try reading for a few seconds
            for _ in 0..2 {
                let result = port.read(&mut buffer);

                // Convert the bytes to a string and check for "macropad"
                match result {
                    Ok(bytes_read) => {
                        // Convert the bytes to a string and check for "macropad"
                        let string = String::from_utf8_lossy(&buffer);
                        println!("{}", string);
                        if string.contains("Lunch_PAD") {
                            println!("Found \"macropad\" on port: {}", port_name);
                            found_macropad = true;
                            break;
                        }
                        if string.contains("Launch_PAD") {
                            println!("Found \"macropad\" on port: {}", port_name);
                            found_macropad = true;
                            break;
                        }
                        if string.contains("PAD") {
                            println!("Found \"macropad\" on port: {}", port_name);
                            found_macropad = true;
                            break;
                        }
                    }
                    Err(e) => println!("Error reading from port: {}", e),
                }

                sleep(Duration::from_millis(100));
            }

            if found_macropad {

                    objects.push(object_information{name:"macropad".to_string(),comport:port_name});

                break;
            } else {
                println!("No \"macropad\" found on port: {}", port_name);
            }
        }




}


use ::image::{GenericImageView, open};

fn draw_pixel(x: f64, y: f64, r: f32, g: f32, b: f32,c: &Context, g2: &mut G2d) {
    // Implementation of drawing a pixel goes here
    // This is a placeholder function
    rectangle([r,g,b, 1.0], // red
              [x, y, 1.0, 1.0],
              c.transform, g2);
}







pub struct pixel_information
{
     pub x: f64,
     pub y: f64,
     pub r: f32,
     pub g: f32,
     pub b: f32,

}



static PIXELS: Mutex<Vec<pixel_information>> = Mutex::new(vec![]);

fn display_image(filename: &str,c: &Context, g2: &mut G2d) {
    // Use the image crate explicitly to open the image
    let mut pixels = PIXELS.lock().unwrap();

    let img = open(filename).expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Iterate over each pixel in the image
    for y in 0..height {
        for x in 0..width {
            // Get the pixel at coordinates (x, y)
            let pixel = img.get_pixel(x, y);

                // pixel.0 is an array [r, g, b, a], so access elements by index
                let r = pixel.0[0];
                let g = pixel.0[1];
                let b = pixel.0[2];


                    pixels.push(pixel_information{x:x as f64,y:y as f64,r:usize_to_float(r),g:usize_to_float(g),b:usize_to_float(b)});

            // Call the draw_pixel function

        }
    }
}

fn usize_to_float(value: u8) -> f32 {
    return (255.0 /value as f32);
}
