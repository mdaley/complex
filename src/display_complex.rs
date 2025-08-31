use num_complex::Complex;

pub trait ComplexDisplay {
    fn to_std_string(&self) -> String;
    fn to_polar_string(&self) -> String;
}

impl ComplexDisplay for Complex<f64> {
    fn to_std_string(&self) -> String {
        if self.im == 0.0 {
            format!("{{{}}}", self.re)
        } else if self.re == 0.0 {
            format!("{{{}}}", im_to_string(self.im))
        } else if self.im < 0.0 {
            format!("{{{} - {}}}", self.re, im_to_string(-self.im))
        } else {
            format!("{{{} + {}}}", self.re, im_to_string(self.im))
        }
    }

    fn to_polar_string(&self) -> String {
        let r = ((self.im * self.im) + (self.re * self.re)).sqrt();
        let theta = self.im.atan2(self.re);
        
        format!("@{{{}, {}}}", r, theta)
    }
}

fn im_to_string(im: f64) -> String {
    match im {
        1.0 => "i".to_owned(),
        -1.0 => "-i".to_owned(),
        _ => format!("{}i", im).to_owned()
    }
}