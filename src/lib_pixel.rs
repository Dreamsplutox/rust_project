#[cfg(test)]
/// Extern a commenter pour lancer les tests cargo test
extern crate lib_pixel;
pub mod pixel{

	#[derive(Debug, Clone, Copy)]
	/// création de la structure avec les valeurs red, green et blue
	pub struct Pixel{
	    red : u8,
	    green : u8,
	    blue : u8
	}

	impl Pixel{
		/// constructeur pour notre pixel
		pub fn new(red : u8, green: u8, blue:u8) -> Self{
	        Pixel{
	            red:red,
	            green:green,
	            blue:blue,
	        }
	    }
		
		/// getters
	    pub fn red(self) -> u8{
	        self.red
	    }

	    pub fn green(self) -> u8{
	        self.green
	    }
	    
	    pub fn blue(self) -> u8{
	        self.blue
	    } 

		/// renvoi le pixel sous forme d'une String (r,g,b), utilisée pour l'affichage stdout
		pub fn display(self) -> String{
	    	format!("({},{},{})",self.red,self.green,self.blue)
	    }

		/// inversion bit à bit
	    pub fn invert(&mut self){
	    	self.red = !self.red;
	    	self.green = !self.green;
	    	self.blue = !self.blue;
	    }

		/// moyenne des valeurs rgb puis répartition ==> transformation en niveau de gris
	    pub fn grayscale(&mut self){
	    	let gray = ((self.green as u16 + self.red as u16 + self.blue as u16) / 3) as u8;
	    	self.red = gray;
	    	self.green = gray;
	    	self.blue = gray;
	    }
		
		/// display pour avoir la même chose que le fichier .ppm
		pub fn display_to_byte(self) -> String{
	    	format!("{} {} {}",self.red,self.green,self.blue)
	    }
	}
	impl PartialEq for Pixel {
		/// vérifie l'égalité de 2 pixels de même nature ==> rgb ou niveau de gris
		fn eq(&self, other:&Pixel) -> bool{
	    	(self.red() == other.red()) && (self.green() == other.green()) && (self.blue() == other.blue())
	    }
	}

	/// test sur grayscale et inversion sur un Pixel

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