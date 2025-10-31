fn main(){
        println!("format {} {} arguments", "some", "another"); //prints "format some another arguments" the braces acts as a placeholder

    //data tyypes in rust
    //just as in javascript use the let keyword to declare a variable
    //always end each line with ";"
    let my_string = "Ihemedo Chinedu Excel";
    let my_age = 19;
    let is_boolean = false;

    println!("my name is: {}", my_string);
    println!("my age is: {}", my_age);
    println!("Is {} the best at anything? {}", my_string, is_boolean);

    //bit sizes and integer types
    //i means signed (positive and negative integers) u means unsigned (positive only)
    //length of an array should always be unsigned as it shouldnt be negative

    // let age:u8 = 300; //error: 8 bits can only store 2^8(FOR UNSIGNED INTEGER TYPES) range of values 
    
    // let mut age_2:u8 = 255; //mut signifies that this variable value can be mutated(changed)
    // println!("{}",age_z2);
    // age_2 = age_2 + 1;
    // // println!("{}",age_2); // rust basically panicked because age_2 has reached its bit range

    //for signed integer values 
    // let age_3:i8 = 255; //error: 8 bits signed can only store -128-127(2^8-1)
    // println!("{}",age_3);


    //i size and u_size, basically rust automatically converts the bits based on the size(rust uses 32bits, if nothing is set)


    //floats 
    //floats can be classified into two types f32 and f64
    //f32 representation becomes corrrect after first 7 decimnal digits before using rounded zeros
    //f64 the representation is more precise and shows to 15 decimal places before rounding zeros
    //rust is f64 by default
    let result = 10.00;
    let interest:f32 = 8.747377373737373737;
    let cost:f64 = 15.28282838484828383;
    println!("default {}", result);
    println!("f32 {}", interest);
    println!("f64 {}", cost);

    //for easy readabbility a number seperator can be used
    let long_number:usize = 50_000_000_000;
    println!("{}", long_number);

    //data types in rust
    //a variable datatype is what determunes the size, layout and values that can be stored in a variable(like typescript)
    // a variable is immutable by default in rust, but the mut keyword makes it mutable
    let _immutable_number = 180; //cannot be changed by default
    //immutable_number = 200  //produces an error
    let mut mutable_number = 20_000;
    println!("mutable {}", mutable_number);
    mutable_number = 15_000;
    println!("mutable_2 {}", mutable_number);

    //const declaration keyword- this is used for variables that CANNOT BE CHANGED.
    const NUMBER_LIMIT:i32 = 10_000; //please note: for variables decleared with the const keyword, their datatype must be set.
    println!("number limit {}", NUMBER_LIMIT);


    //this blew my mind but rust allows to declare variables with the same name,
    //but in such case the new variable overrides the previous
    //keep in mind this not the same as creating a mutable type, as two different variables are stored in memory
    // let salary = 100; //this will notify that the variable wasnt used at all.
    let salary = 0.50;
    println!("salary {}", salary);

    //constants cannot be shadowed


    //STRINGS
    //strings can be classified to two data types
    //string literal(&str) vs string object(String)

    //sring literals 
    let company:&str = "Hello world";
    println!("company {}", company);
    
    //string literals are static by default, basically meaning that the string literals are guaranteed to be vallid for the duration of the entire program.
    //explicitly written
    let company:&'static str = "Hello world";
    println!("company {}", company);

    //string objects
    //String Object type is a standard library and is not a core part of thye language
    //it is mutable and utf8-encoded type and it can be used to represent values that will be provided during runtime


    let empty_string = String::new(); //empty string object
    let from_string = String::from("ihemedo chindu"); //passes an argument to the from menthod

    println!("empty{}, from{}", empty_string.len(), from_string.len());
 
    //push_string() method
    let mut b = String::new();
    b.push_str("hello there");
    println!("push_str {}", b);
    
    //replace() method
    //quite different from the as_string() and to_string() as it creates a mutable copy of the string 
    let d = String::from("Ihemedo Chinedu");
    let mut replace_string = d.replace("Chinedu","Excel");
    replace_string.push_str("Nathan");
    println!("replace {}", replace_string);

    //as_str() method is the opposite of to_string()
    //converts the owned string to a borrowed string
    //note: it doesnt create a new copy or fresh allocation, it just creates a read-only(&str) pointer to the owned String
    //and &str is immutable please note that you cannot change the value of the variable "new"
    let e = String::from("Ihemedo Chinedu");
    let new = e.as_str();
    println!("converting from owned String to a borrowed &str view {}", new);
  
    //push() method - different from push_str
    //used to append a char to the end of the string, and '' single quote.
    let mut z = String::from("Ihemedo Chinedu");
    z.push('.');
    println!("fullname: {}", z);

    //split_whitespace() method splits(based on whitespaces " ", \n, \t) a string and returns an iterator
    //NOTE: an iterator, not an array.
    let this_string = String::from("Nathan medo is a friend");
    let iter_string = this_string.split_whitespace();
    // println!("iter: {:?}", iter_string.next());
    // println!("iter: {:?}", iter_string.next());
    // println!("iter: {:?}", iter_string.next());
    // println!("iter: {:?}", iter_string.next());
    let mut i = 1;
    //An ideal example for split_whitespaces, since it doesnt return an array but a ready to use loop
    //remember, even though it might be tempting to say inter_string is not an array, but an iteration.
    for token in iter_string{
        println!("token {}. {}", i, token);
        i+=1;
    }

    //split() method is just like the split_whitespaces() method except that it actually accepts argument.
    //still doesnt return an array but an iteration
    let static_bulky_string = "Nathan,is,a,sily,boy";
    //convert to owned string
    let mut owned_bulky_string = static_bulky_string.to_string();
    owned_bulky_string.push_str(",right?");
    let array_string:Vec<&str> = static_bulky_string.split(",").collect();
    println!("array: {:?}", array_string);
    for word in owned_bulky_string.split(","){
        println!("Terms: {}.", word); 
    }

    //cannot be mutated even if mut is added, recall.
    //however, please do not mistake mutate to reassign/replace. A variable storing a borrowed string can still be reassigned a new value
    //but the current string it stores cannot be mutated by any mutate methods.
    let mut name1:&str = "Nathanmedo"; 
    println!("name {}", name1);
    name1 = "Ihemedo chinedu";
    println!("name {}", name1);
    
    //to_string() converts a borrowed view to a owned string
    let static_string = "Hello world";
    let mut object_string = static_string.to_string();
    object_string.push_str("There!");
    println!("modified string object {}", object_string)


    //you may have thought of this, keep in mind this fails 
    //reason: const are used to store constant expressions and not result of a function or a value computed at run time.
    //String::from() is already a inbuilt function called.
    // const THIS_VARIABLE:&str = String::from("ihemedo chinedu");
    // println!("here {}", THIS_VARIABLE)

}