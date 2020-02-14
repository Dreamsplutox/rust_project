#[cfg(test)]

extern crate lib_pixel;
pub mod pixel{

	#[derive(Debug, Clone, Copy)]
	pub struct Pixel{ //création de la structure
	    red : u8,
	    green : u8,
	    blue : u8
	}

	impl Pixel{
	    pub fn new(red : u8, green: u8, blue:u8) -> Self{ //constructeur
	        Pixel{
	            red:red,
	            green:green,
	            blue:blue,
	        }
	    }
	    
	    pub fn red(self) -> u8{ //retourne la valeur rouge
	        self.red
	    }

	    pub fn green(self) -> u8{ //retoure la valeur vert
	        self.green
	    }
	    
	    pub fn blue(self) -> u8{ //retourn la valeur bleu
	        self.blue
	    } 

	    pub fn display(self) -> String{ // renvoi le pixel sous forme d'une String (r,g,b)
	    	format!("({},{},{})",self.red,self.green,self.blue)
	    }

	    pub fn invert(&mut self){ //inversion bit à bit
	    	self.red = !self.red;
	    	self.green = !self.green;
	    	self.blue = !self.blue;
	    }


	    pub fn grayscale(&mut self){ //moyenne des valeurs rgb puis répartition ==> transformation en niveau de gris
	    	let gray = ((self.green as u16 + self.red as u16 + self.blue as u16) / 3) as u8;
	    	self.red = gray;
	    	self.green = gray;
	    	self.blue = gray;
	    }
	    pub fn display_to_byte(self) -> String{ //display pour avoir la même chose que le fichier .ppm
	    	format!("{} {} {}",self.red,self.green,self.blue)
	    }
	}
	impl PartialEq for Pixel {
	    fn eq(&self, other:&Pixel) -> bool{ //vérifie l'égalité de 2 pixels de même nature ==> rgb ou niveau de gris
	    	(self.red() == other.red()) && (self.green() == other.green()) && (self.blue() == other.blue())
	    }
	}

	#[test]
	pub fn test_pixel_grayscale(){
		let val =((34 + 56 + 102) / 3) as u8;
		let mut pixel_origin = Pixel::new(34,56,102);
		pixel_origin.grayscale();
		let pixel_gray = Pixel::new(val,val,val);
		assert_eq!(pixel_origin, pixel_gray);
	}
	#[test]
	pub fn test_pixel_invert(){
		let val1 = 102;
		let val2 = 128;
		let val3 = 200;
		let mut pixel_origin = Pixel::new(val1, val2, val3);
		pixel_origin.invert();
		let pixel_invert = Pixel::new(!val1, !val2, !val3);
		assert_eq!(pixel_origin, pixel_invert);
	}

}