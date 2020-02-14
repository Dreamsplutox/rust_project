use std::path::Path;
use std::fs;
mod lib_image;
use crate::lib_image::image::*;
mod lib_pixel;
use crate::lib_pixel::pixel::*;

fn new_with_file(filename: &str)->Image{ //creation struct image avec ouverture fichier
	println!("In file {}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file"); // tentative de lecture du fichier

    let my_lines = content.split("\n").collect::<Vec<_>>(); // iterator ==> contient toutes les lignes du fichier

    let format = my_lines[0].to_string(); //première ligne en string ==> le format
    let width_height = my_lines[1].split_whitespace().collect::<Vec<_>>(); //2ième ligne : les dimmensions ==> séparation pour avoir de nouveau un itérator
    //récupéation des dimmensions
    let width = width_height[0].parse::<usize>().unwrap(); 
    let height = width_height[1].parse::<usize>().unwrap();


    let max = my_lines[2].parse::<u8>().unwrap(); //récupération du max dans la 3ième ligne

    let mut pixels_vec = Vec::<Pixel>::new(); //création d'un vec de Pixels

    let pixels_lines = &my_lines[3..]; //récupération de tout les pixels (à partir de la 4ième ligne)

    for pixels_line in pixels_lines{ //parcour des lignes
    	let line = pixels_line.split_whitespace().collect::<Vec<_>>(); // récupération d'une ligne en itérator
    	let mut i = 0; //conteur de pixels
    	while i < width*3 { //width * 3 pour savoir quand est-ce que la ligne est terminée (3 données par pixels)
    		let r = line[i].parse::<u8>().unwrap(); //récupéation du rouge
    		let g = line[(i+1)].parse::<u8>().unwrap(); //récupéation du vert
    		let b = line[(i+2)].parse::<u8>().unwrap(); //récupéation du bleu
    		pixels_vec.push(Pixel::new(r,g,b)); //création et insertion du Pixel dans le vec
    		i += 3; //incrémentation de 3 pour changer de pixel
    	}
    }
    
    Image::new(format,width,height,max,pixels_vec) //création de l'image via son new
} 

fn main() {

    let filename = "D:\\Documents\\4ieme_annee\\langage_Rust\\Code\\project\\src\\picture_P3.ppm";
    let mut saved_image:Image = new_with_file(filename);

    let mut my_image:Image = new_with_file(filename);
    let mut my_image2:Image = new_with_file(filename);

    let mut my_image_invert:Image = new_with_file(filename);
    let mut my_image_grayscale:Image = new_with_file(filename);
    
  	my_image_invert.invert();

  	//println!("{}",my_image1.eq(my_image));

  	my_image_grayscale.grayscale();

	//println!("{}",my_image1.eq(my_image2));

    let path = Path::new("D:\\Documents\\4ieme_annee\\langage_Rust\\Code\\project\\src\\test.ppm");
    
    saved_image.save(path);




}