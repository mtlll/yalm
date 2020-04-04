impl Converse for Inputs<'_> {
    
    fn prompt_echo(&mut self, _: &CStr) -> Result<CString, ()> {
         unimplemented!()
    }
    
    fn prompt_blind(&mut self, _: &CStr) -> Result<CString, ()> {
        unimplemented!()
    }
    
    fn info(&mut self, _: &CStr) {
        unimplemented!()
    }
    
    fn error(&mut self, _: &CStr) {
        unimplemented!()
    }
    
    fn username(&self) -> &str {
        unimplemented!() 
    }
}