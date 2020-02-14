#[cfg(test)]
/// Extern commenter pour lancer les tests cargo test
extern crate lib_image;
pub mod image{
	use std::path::Path;
	use std::fs::File;
	use std::io::prelude::*;
	use crate::lib_pixel::pixel::*;

	#[derive(Debug)]
	/// Notre structure image avec comme format P3 ou P6 (enum), les dimensions de
	/// l'image ainsi que la valeur maximale pour chaque couleur (255 par ex)
	pub enum Format{
        P3,
        P6,
    }

	#[derive(Debug)]
	pub struct Image{ // structure image
	    format: Format, // format
	    width: usize, 
	    height: usize,
	    max: u8, // val max pour chaques couleurs ==> u8 car max : 255
	    pixels: Vec<Pixel>,
	}

	impl Image{
		/// constructeur de l'image 
		pub fn new(format:Format, width:usize, height:usize, max:u8, pixels:Vec<Pixel>) -> Self{ //constructeur
	        Image{
	            format: format, 
			    width: width, 
			    height: height,
			    max: max,
			    pixels: pixels,
	        }
	    }

		/// sauvegarde la structure dans un fichier à l'adresse "path"
		/// * on commence d'abord par récupérer tout ce qui n'est pas un pixel dans une String
		/// nommée header
		/// * on crée le fichier (on affiche une erreur s'il y a un problème) 
		/// * on insère ensuite le header dans le fichier (gestion de l'erreur)
		/// * on parcours ensuite notre vecteur de pixels
		/// * on écrit les pixels, on rajoute un espace et quand on atteint width
		/// * on affiche le chemin du fichier créé
		pub fn save(self, path: &Path){
			let mut count:usize = 0; 
			let header = format!( 
	            "{:?}\n{} {}\n{}\n",
	            self.format, self.width, self.height, self.max
	    	);

		    let mut file = match File::create(path) {
		        Err(why) => panic!("couldn't create file"),
		        Ok(file) => file,
		    };

		    match file.write_all(header.as_bytes()) {
		        Err(why) => panic!("couldn't write header to file"),
		        Ok(_) => println!("successfully wrote header : \n{}", header),
		    }

		    for pixel in self.pixels {
		        match file.write_all(pixel.display_to_byte().as_bytes()) {
			        Err(why) => panic!("couldn't write pixel to file"),
			        Ok(_) => print!("{}", pixel.display()),
			    }
			    match file.write_all(" ".as_bytes()) {
			        Err(why) => panic!("couldn't write 'space' to file"),
			        Ok(_) => print!(" "),
			    }
			    count += 1;
		        if count == self.width {
		        	match file.write_all("\n".as_bytes()) {
				        Err(why) => panic!("couldn't write '\\n' to file"),
				        Ok(_) => println!("\n"),
				    }
		        	count = 0;
		        }		        
		    }
		    println!("#######################################################################################");
		    println!("file writen to \"{}\"", path.display());
			println!("#######################################################################################");
		}

		/// inversion de tout les pixels ==> utilisation de la fonction Pixel.invert()
		pub fn invert(&mut self){
			for pixel in self.pixels.iter_mut(){
				pixel.invert();
			}
	    }

		/// passage de tous les pixels en niveau de gris ==> utilisation de la fonction Pixel.grayscale()   
		pub fn grayscale(&mut self){
	    	for pixel in self.pixels.iter_mut(){
				pixel.grayscale();
			}
	    }

		/// getters de nos attributs
	    pub fn width(&self)->usize{
	    	self.width
	    }
	    pub fn height(&self)->usize{
	    	self.height
	    }
	    pub fn max(&self)->u8{
	    	self.max
	    }
	    pub fn pixels(&self)->& Vec<Pixel>{
	    	& self.pixels
	    }

		/// vérifie que notre image est identique à une autre en comparant les Pixels
	    pub fn eq(self, other: Image) -> bool{
	    	self.pixels().eq(other.pixels())
	    }

	}

	/// tester l'application de grayscale et invert sur nos images
	#[test]
	pub fn test_image_invert(){
		let mut image_origin = Image::new(Format::P3, 2, 1, 255, vec![Pixel::new(34,56,102), Pixel::new(42,75,255)]);
		let mut image_invert = Image::new(Format::P3, 2, 1, !34, vec![Pixel::new(!34,!56,!102), Pixel::new(!42,!75,!255)]);
		
		image_origin.invert();

		assert_eq!(image_origin.pixels[0], image_invert.pixels[0]);
		assert_eq!(image_origin.pixels[1], image_invert.pixels[1]);
	}


	#[test]
	pub fn test_image_grayscale(){
		let val1 = ((34+56+102)/3) as u8;
		let val2 = ((42+75+255)/3) as u8;
		let mut newmax = 0;
		if (34+56+102)/3 > (42+75+255)/3 {
			newmax = ((34+56+102)/3) as u8;
		}else{
			newmax = ((42+75+255)/3) as u8;
		}
		let mut image_origin = Image::new(Format::P3, 2, 1, 255, vec![Pixel::new(34,56,102), Pixel::new(42,75,255)]);
		let mut image_grayscale = Image::new(Format::P3, 2, 1, newmax, vec![Pixel::new(val1,val1,val1), Pixel::new(val2,val2,val2)]);
		
		image_origin.grayscale();

		assert_eq!(image_origin.pixels[0], image_grayscale.pixels[0]);
		assert_eq!(image_origin.pixels[1], image_grayscale.pixels[1]);
	}

}

 