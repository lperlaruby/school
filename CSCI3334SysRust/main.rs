fn main(){
    //Shadowing
    let x: 132 = 5;
    //let x: &str = "word";
    {
        let x: 132 = x + 1; //Creates a new variable
    }
    //Mutation
    let mut y: 132 = 5;
    y = y + 1; //Modifies the existing variable

    //x = "word";

    println!("x: {}, y: {}", x, y);
    //println!("The value of x is: {}", x);
    
    //uncommenting the next line will result in a compilation error x = 6; // error: cannot assign twice to immutable variable 'x'
}