use image::GenericImageView;

fn main() {
    
    let (char_vector, width, height) = get_ascii("rust.png");
    
    print_vec(char_vector, width, height);
}

fn map_value(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
    // Ensure that x is within the source range
    let x = x.min(x_max).max(x_min);

    // Perform the linear interpolation
    let y = (x - x_min) * (y_max - y_min) / (x_max - x_min) + y_min;

    y
}

fn get_ascii(dir: &str) -> (Vec<char>, u32, u32){ // Vec, x, y
    // Ascii characters to determine the
    let ascii_karakterleri = "   .'`^,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    // a
    let ascii_vec_len : f32 = ascii_karakterleri.len() as f32;
    // Convert the string to a vector of characters
    let ascii_chars: Vec<char> = ascii_karakterleri.chars().collect();

    let img = image::open(dir).unwrap();
   
    let (width, height) = img.dimensions();

    let mut brightness_values: Vec<f32> = Vec::new();
    let mut ascii_values: Vec<char> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            // Get pixel from coordinates
            let rgba = img.get_pixel(x, y); 
            
            // Calculate pixel brightness
            let pixel_brightness = (rgba[0] as f32 * 0.21) + (rgba[1] as f32 * 0.72) + (rgba[2] as f32 * 0.07);
            
            //println!("{:.2}", pixel_brightness);
            
            // This can be removed maybe can be used to debug so it is left here.
            brightness_values.push(pixel_brightness);
            
            // Map useing Linear interpolation
            let corresponding_ascii = map_value(pixel_brightness, 0.0, 255.0, 0.0, ascii_vec_len - 1.0);
            
            // Push the correct char to ascii_values
            ascii_values.push(ascii_chars[corresponding_ascii as usize]);
        }
    }

    (ascii_values, width, height)
    
    //println!("Hello, world! {:?}", pixels);
}

fn print_vec(img_chars: Vec<char>, width: u32, height: u32){
    for y in 0..height-1 {
        for x in 0..width-1 {
            // print the lines
            // y * width + x is for changing the y coordinate by adding the current y value times
            // total width
            print!("{}", img_chars[(y * width + x) as usize]);
        }
        println!();
    }
}
