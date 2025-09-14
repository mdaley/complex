use crate::complex::Complex;


trait ComplexOperation  {
    fn call(&self, args: &[Complex]) -> Result<Complex, String>;
}

/*struct Add;
impl ComplexOperation for Add {
    fn call(&self, args: &[Complex]) -> Result<Complex, String> {
        let r = args.get(0).unwrap() + args.get(1).unwrap();
        Err("".to_string())
    }
}*/