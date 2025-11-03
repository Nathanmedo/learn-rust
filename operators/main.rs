fn main(){
    let num1:i32 = 10;
    let num2:i32 = 20;
    let mut result:i32;

    //addition operation
    result = num1 + num2;
    println!("Addition: {}", result);

    //substraction operation
    //please recall: the result was set to a signed 32 bit which means 0 and above till 2^32-1
    result = num2 - num1;
    println!("Substraction {}", result);

    //product operation
    result = num1 * num2;
    println!("Product {}", result);

    //division operation
    result = num2 / num1;
    println!("Division {}", result);
    //modulus or remainder operator
    result = num2 % num1;
    println!("remainder {}", result);

    //relational operators
    //relational operators are used to define the relationship between two or more entities
    //in basic terms, compare a value or element with respect to another - it returns boolean
    //these are the same in most programming languages
    let mut is_true:bool;

    println!("value of A {}", num1);
    println!("value of B {}", num2);

   //greater than >
    is_true = num1 > num2;
    println!("A>B {}", is_true);

    //less than <
    is_true = num1 < num2;
    println!("A<B {}", is_true );
    
    //greater than or equal to
    is_true = num1 >= num2;
    println!("A>=B {}", is_true );
    
    //less than or equal to
    is_true = num1 <= num2;
    println!("A<=B {}", is_true );
    
    //is equal to
    is_true = num1 == num2;
    println!("A==B {}", is_true );
    
    //logical operators - this is also a basic concept in programming
    //they are used to combine two or more conditions together.
    
    is_true = num1 > 10 && num2 > 10;
    println!("condition {}", is_true );

    //if statements
    let a = 40;
    let b = 30;

    //it can be written with or without the brackets, but for clean and understandable code we use brackets
    if (a>10) && (b>10){
        println!("true");
    }

    //or operator
    //note in this example b is not greater than 30 but the condition still runs
    if (a > 10) || (b > 100){
        println!("true")
    }
    
    //bitwise operators
    //before you try to grasp this, kindly refresh your memory on truth tables, base10 to binary and binary to decimak cinversuin
    //This binary world you're stepping into is the bedrock under systems programming, cryptography, GPUs, and compilers. This isn’t just Rust — it’s how machines think.

    //And bitwise operator
    let a: i8 = 6; //in binary  110
    let b: i8 = 7; // in binary 111

    //compares the binary form using AND rules and the final binary gotten is then converted back to base10
    let mut result = a & b; // 110 & 111 = 110, 6 is the result
    println!("And bitwise {}", result);
    
    result = a | b; //110 or 111 = 111, 7
    println!("And bitwise {}", result);
    
    //there are more bitwise operators like ^XOR, !Bitwise Not, << left shift, >> right shift, >>> right shift with zero
}