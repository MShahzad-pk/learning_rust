

    use std::io;
    pub fn take_string(y:String)-> String {
        let mut x = String::new();
        println!("Enter name of student {}", y);
        io::stdin().read_line(&mut x).err();
        x
    }
    pub fn take_u8 (value:String)-> u8 {
        let mut x = String::new();
        println!("Enter the age of {} between 0 to 255",value);
        io::stdin().read_line(&mut x).err();
        let y:u8=x.trim().parse().expect("Enter entered non number value");
    y
}

    

