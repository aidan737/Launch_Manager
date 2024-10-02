extern crate piston_window;
pub use piston_window::*;
#[path = "taskbar.rs"] mod Taskbar;


pub fn start_window(title:&str,width: f64, hight: f64) -> PistonWindow {



    let mut window: PistonWindow =
    WindowSettings::new(title, [width, hight])
    .exit_on_esc(true).resizable(false).decorated(false).build().unwrap();

    return window;
}


pub fn clear_screen(c: &Context, g: &mut G2d) {
    clear([1.0; 4], g);
}


fn main() {

    update();
    let mut dragging = false;
    let mut drag_offset = [0, 0];
    let mut window: PistonWindow = start_window("Launch Editor",1000.0, 690.0);
    let mut mouse_button_left = false;

    let mut minimized:bool = false;
     let mut key_pressed = Key::Unknown;
let mut scroll_delta = 0.0; // Add this line to store scroll wheel delta
let mut scroll_distance = 0.0;
let mut scroll_distance2 = 0.0;
   let mut cursor = [0.0, 0.0];
  let mut cursor2 = [0.0, 0.0];
  while let Some(e) = window.next() {
      if (window.window.window.is_minimized() == Some(false)) {
      e.mouse_cursor(|pos| {
          cursor = pos;

      });
      if let Some(Button::Keyboard(key)) = e.press_args() {

            key_pressed = key;

           }
           if let Some(Button::Keyboard(key)) = e.release_args() {

            key_pressed =  Key::Unknown;
           }
      if let Some(Button::Mouse(button)) = e.press_args() {

       if(button == MouseButton::Left)
       {
           mouse_button_left = true;
           if(is_mouse_in_box(cursor[0],cursor[1], 0.0, 0.0, 940.0, 30.0))
           {
           dragging = true;
            drag_offset[0] = cursor[0] as i32;
            drag_offset[1] = cursor[1] as i32;


           }
       }
      }
      if let Some(Button::Mouse(button)) = e.release_args() {

       if(button == MouseButton::Left)
       {
             dragging = false;
           mouse_button_left = false;

       }
     }


     if let Some(args) = e.mouse_scroll_args() {
              scroll_delta = args[1]; // Vertical scroll

              scroll_distance += args[1];
              // You can use scroll_delta in your application logic
          }


     if(dragging && cursor != cursor2)
     {

         window.set_position([(window.get_position().unwrap().x as i32) +(cursor[0] as i32 - drag_offset[0]),(window.get_position().unwrap().y as i32) +(cursor[1]as i32 - drag_offset[1])]);
                     cursor2 = cursor;
     }


     //logic for top taskbar
     if(is_mouse_in_box(cursor[0],cursor[1], 970.0, 0.0, 30.0, 30.0)){
                 if(mouse_button_left){
                      break;
                 }
         }

    if(is_mouse_in_box(cursor[0],cursor[1], 940.0, 0.0, 30.0, 30.0)){
             if(mouse_button_left){
                 window.window.window.set_minimized(true);
                 mouse_button_left = false;
             }

        }
     let window_hight = window.size().height;
     let window_width = window.size().width;

     window.draw_2d(&e, |c, g, _device| {

         //create a top bar


         rectangle([0.3,0.3, 0.3, 1.0], // red
                   [0.0, 0.0, 1000.0, 30.0],
                   c.transform, g);

        //x button
        if(is_mouse_in_box(cursor[0],cursor[1], 970.0, 0.0, 30.0, 30.0)){
            rectangle([0.6,0.3, 0.3, 1.0], // red
                        [970.0, 0.0, 30.0, 30.0],
                        c.transform, g);

            }


           // Calculate the vertices of the X
           let vertices = [
               [970.0, 0.0],
               [980.0, 15.0],
               [970.0, 30.0],
               [975.0, 30.0],
               [985.0, 20.0],
               [995.0, 30.0],
               [1000.0, 30.0],
               [990.0, 15.0],
               [1000.0, 0.0],
               [995.0, 0.0],
               [985.0, 10.0],
               [975.0, 0.0],
           ];
           let triangle1= [
                [970.0, 0.0], [980.0, 15.0], [975.0, 0.0]
                      ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle1, c.transform, g);
           let triangle2= [
           [975.0, 0.0], [980.0, 15.0], [985.0, 10.0]
           ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle2, c.transform, g);
           let triangle3= [
              [985.0, 10.0], [980.0, 15.0], [985.0, 20.0]
           ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle3, c.transform, g);
           let triangle4= [
              [970.0, 30.0], [980.0, 15.0], [975.0, 30.0]
           ];
           polygon([1.0, 1.0, 1.0, 1.0], &triangle4, c.transform, g);
           let triangle5= [
               [975.0, 30.0], [980.0, 15.0], [985.0, 20.0]
           ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle5, c.transform, g);

           let triangle6= [
              [995.0, 30.0], [1000.0, 30.0], [985.0, 20.0]
           ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle6, c.transform, g);
           let triangle7= [
               [1000.0, 30.0], [985.0, 20.0], [990.0, 15.0]
           ];
polygon([1.0, 1.0, 1.0, 1.0], &triangle7, c.transform, g);
           let triangle8= [
               [985.0, 20.0], [990.0, 15.0], [985.0, 10.0]
           ];
           polygon([1.0, 1.0, 1.0, 1.0], &triangle8, c.transform, g);
           let triangle9= [
               [1000.0, 0.0], [985.0, 10.0], [990.0, 15.0]
           ];
           polygon([1.0, 1.0, 1.0, 1.0], &triangle9, c.transform, g);
           let triangle10= [
               [995.0, 0.0], [1000.0, 0.0], [985.0, 10.0]
           ];
           polygon([1.0, 1.0, 1.0, 1.0], &triangle10, c.transform, g);
           // Draw the X using the polygon function
           //polygon([1.0, 1.0, 1.0, 1.0], &vertices, c.transform, g);




//- minimize button

                if(is_mouse_in_box(cursor[0],cursor[1], 940.0, 0.0, 30.0, 30.0)){
                    rectangle([0.6,0.3, 0.3, 1.0], // red
                                [940.0, 0.0, 30.0, 30.0],
                                c.transform, g);

                    }

                    rectangle([1.0,1.0, 1.0, 1.0], // red
                            [940.0, 15.0, 30.0, 5.0],
                                c.transform, g);








            if( Taskbar::render(&c,g,cursor,mouse_button_left,key_pressed,minimized,scroll_delta) != 0){
             clear_screen(&c,g);
             scroll_delta = 0.0;
            }

            minimized = false;

         });
     }else
     {
         minimized = true;
     }

  }
}
fn is_mouse_in_box(mouse_x: f64, mouse_y: f64, box_x: f64, box_y: f64, box_width: f64, box_height: f64) -> bool {
    mouse_x >= box_x && mouse_x < box_x + box_width && mouse_y >= box_y && mouse_y < box_y + box_height
}






use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
fn update() {
    let download_url = "https://drive.google.com/uc?export=download&id=1elIulkRI0rajvZaNiroGiE426F0gLuUK";
    let current_version = "0.0.2";
      let local_file_path = "Update.zip";
   // Download the EXE file

   let result = download_file(download_url);

   match result {
       Ok(content) => {   println!("{ }", String::from_utf8_lossy(&content));
          // Save the downloaded EXE file
          if(String::from_utf8_lossy(&content) != current_version)
          {
              //now update
                  let download_url2 = "https://drive.google.com/uc?export=download&id=1oxZNWuhq5cdzioSc1RIWVTRr76RA13Lb";
                  let response2= Client::new().get(download_url2).send().unwrap();
                  let content2 = response2.bytes().unwrap();

                  let mut file = File::create(local_file_path).unwrap();
                  file.write_all(&content2).unwrap();
                  updateexe(&(r"Update\Launch_Manager.exe"));




          }
      },
       Err(error) => println!("Error downloading file: {}", error),
   }


}

fn download_file(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut content = Vec::new();
    response.read_to_end(&mut content)?;

    Ok(content)
}

use update_me;
use zip_extensions::*;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
pub fn updateexe(path: &str) {

extract_entry_to_memory();


}
fn extract_entry_to_memory() {
    let archive_file = PathBuf::from_str(r#"Update.zip"#);
    let entry_path = PathBuf::from_str("Launch_Manager.exe");

    let mut buffer : Vec<u8> = vec![];
    match zip_extract_file_to_memory(&archive_file.unwrap(), &entry_path.unwrap(), &mut buffer) {
        Ok(()) => { println!("Extracted {} bytes from archive.", buffer.len()) },
        Err(e) => { println!("The entry does not exist.") }
    };
    update_me::apply(&mut buffer);
}
