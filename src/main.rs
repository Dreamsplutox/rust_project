use std::path::Path;
use std::fs;
mod lib_image;
use crate::lib_image::image::*;
mod lib_pixel;
use crate::lib_pixel::pixel::*;

/// creation struct image avec ouverture fichier
/// * tentative de lecture du fichier
/// * creation d'un iterator pour parcourir le fichier (ligne par ligne)
/// * conversion de notre première ligne en string
/// * resplit sur l'espace de la lgine pour récupérer notre width et height
/// * récupération du max sur la troisième ligne
/// * création d'un vecteur de pixels et récupérer le reste du fichier (à partir de la ligne 4)
/// * récupération des valeurs en itérant sur la ligne
/// * création de l'image en utilisant le constructeur
fn new_with_file(filename: &str)->Image{
	println!("In file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let my_lines = content.split("\n").collect::<Vec<_>>();

    let format = match my_lines[0] {
        "P3" => Format::P3,
        "P6" => Format::P6,
        _ => panic!("FORMAT NOT ACCEPTED"),
    };
    let width_height = my_lines[1].split_whitespace().collect::<Vec<_>>();
    
    let width = width_height[0].parse::<usize>().unwrap(); 
    let height = width_height[1].parse::<usize>().unwrap();


    let max = my_lines[2].parse::<u8>().unwrap(); 

    let mut pixels_vec = Vec::<Pixel>::new(); 

    let pixels_lines = &my_lines[3..];

    for pixels_line in pixels_lines{
    	let line = pixels_line.split_whitespace().collect::<Vec<_>>();
    	let mut i = 0; 
    	while i < width*3 {
    		let r = line[i].parse::<u8>().unwrap();
    		let g = line[(i+1)].parse::<u8>().unwrap();
    		let b = line[(i+2)].parse::<u8>().unwrap();
    		pixels_vec.push(Pixel::new(r,g,b));
    		i += 3;
    	}
    }
    
    Image::new(format,width,height,max,pixels_vec)
} 

fn main() {
    /// création et sauvegarde d'une image
    let filename = "C:\\Users\\arnau\\Desktop\\quatrième_année\\programmation rust\\Projet\\projet_rust\\rust_project\\src\\picture_P3.ppm";
    let mut saved_image:Image = new_with_file(filename);

    let path = Path::new("C:\\Users\\arnau\\Desktop\\quatrième_année\\programmation rust\\Projet\\projet_rust\\rust_project\\src\\test.ppm");
    saved_image.save(path); 

}