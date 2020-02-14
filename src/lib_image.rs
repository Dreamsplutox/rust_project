#[cfg(test)]

extern crate lib_image;

pub mod image{
	use std::path::Path;
	use std::fs::File;
	use std::io::prelude::*;
	use crate::lib_pixel::pixel::*;

	#[derive(Debug)]
	pub enum format{
        P3,
        P6,
    }

	#[derive(Debug)]
	pub struct Image{ // structure image
	    format: format, // format
	    width: usize, 
	    height: usize,
	    max: u8, // val max pour chaques couleurs ==> u8 car max : 255
	    pixels: Vec<Pixel>,
	}

	impl Image{
		pub fn new(format:format, width:usize, height:usize, max:u8, pixels:Vec<Pixel>) -> Self{ //constructeur
	        Image{
	            format: format, 
			    width: width, 
			    height: height,
			    max: max,
			    pixels: pixels,
	        }
	    }

		pub fn save(self, path: &Path){ // sauvegarde la structure dans un fichier à l'adresse "path"
			let mut count:usize = 0; //compteur 
			let header = format!( //récupère tout ce qui n'est pas des pixels et le met sous forme d'une String
	            "{:?}\n{} {}\n{}\n",
	            self.format, self.width, self.height, self.max
	    	);

		    let mut file = match File::create(path) { // crée le fichier, si erreur ==> print message erreurs
		        Err(why) => panic!("couldn't create file"),
		        Ok(file) => file,
		    };

		    match file.write_all(header.as_bytes()) { //insère le header dans le fichier, si erreur ==> message erreur, sinon ==> message réussite
		        Err(why) => panic!("couldn't write header to file"),
		        Ok(_) => println!("successfully wrote header : \n{}", header),
		    }

		    for pixel in self.pixels {  // parcour tout les pixels de la structure
		        match file.write_all(pixel.display_to_byte().as_bytes()) { // tentative d'écriture du pixel sous la forme "r g b" dans le fichier, si erreur ==> message erreur
			        Err(why) => panic!("couldn't write pixel to file"),
			        Ok(_) => print!("{}", pixel.display()),
			    }
			    match file.write_all(" ".as_bytes()) { //mise d'un espace pour la mise en forme
			        Err(why) => panic!("couldn't write 'space' to file"),
			        Ok(_) => print!(" "),
			    }
			    count += 1;
		        if count == self.width { // mise d'un \n pour la mise en forme
		        	match file.write_all("\n".as_bytes()) {
				        Err(why) => panic!("couldn't write '\\n' to file"),
				        Ok(_) => println!("\n"),
				    }
		        	count = 0;
		        }		        
		    }
		    println!("#######################################################################################");
		    println!("file writen to \"{}\"", path.display()); //affiche le chemin de sauvegarde du fichier
			println!("#######################################################################################");
		}

		pub fn invert(&mut self){ //inversion de tout les pixels ==> utilisation de la fonction Pixel.invert()
			for pixel in self.pixels.iter_mut(){
				pixel.invert();
			}
	    }

	   	pub fn grayscale(&mut self){ //passage de tout les pixels en niveau de gris ==> utilisation de la fonction Pixel.grayscale()
	    	for pixel in self.pixels.iter_mut(){
				pixel.grayscale();
			}
	    }

	    pub fn width(&self)->usize{
	    	self.width
	    }
	    pub fn height(&self)->usize{
	    	self.height
	    }
	    pub fn max(&self)->u8{
	    	self.max
	    }
	    pub fn pixels(self)->Vec<Pixel>{
	    	self.pixels
	    }

	    pub fn eq(self, other: Image) -> bool{
	    	self.pixels().eq(&other.pixels())
	    }

	}
}

 