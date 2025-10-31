fn main(){
    //string data types are classified into two types in rust
    //string literal(&str)
    //string Object(String)

    //string literals- they are immutable string types
    //Meaning mutable methods like push, push_str etc. doesnt work on it
    //kindly note: the mut keyword only makes the variable mutable not the value of the variable mutable 
    //meaning the variable can be reassigned -HUGE DIFFERENCE
    let mut my_name: &str = "Ihemedo Chinedu";
    println!("Name: {}", my_name);
    //because the variable is mutable "mut" can be reassigned to another string
    my_name = "Ihemedo Nathan";
    println!("Name: {}", my_name);

    //String Object;
    //creating a new string object
    //the mut keyword is also needed here, despite the use of the string object, why?
    //trying to mutate its value cannot be done without making the variable mutable
    //think of it like a two factor authentication, the mut already makes the variable
    //using a string type makes it a mutable string type
    let mut my_class = String::new();
    my_class.push_str("Nathan");
    println!("Class: {}", my_class);

    //common string methods
    //to_string is used to convert a string literal(Immutable) to a string Object(Mutable)
    //it can also be said convert an borrowed string view (&str) to a Owned string (String)
    let my_age: &str = "Hello, I'm 18!";
    let my_age_year:i32 = 2007;
    println!("Age: {}", my_age);
    
    //mutable string methods
    //converting to string object
    //please note: it can also be used for converting number types to string types
    let mut my_age_string = my_age.to_string();
    let my_age_year_string = my_age_year.to_string();
    println!("Age Year: {}", my_age_year_string);


    //remove method .remove(index)
    my_age_string.remove(1);
    println!("Age: {}", my_age_string);
    
    //.push_str(string)-append a string slice
    my_age_string.push_str("Nathan is 18");
    println!("Age: {}", my_age_string);
    
    //.push(ch) - append a single character
    //please note: use a single quote string for single character append.
    my_age_string.push('!');
    println!("Age: {}", my_age_string);
    
    //.insert(index, ch) - insert a single character
    my_age_string.insert(1, 'e');
    println!("Age: {}", my_age_string);
    
    //insert_str(index, str) -insert a string slice
    //please note that it INSERTS and not replace - So for the index of 4 it adds the text after the index
    my_age_string.insert_str(4, "ooooo");
    println!("Age: {}", my_age_string);


    //immutable methods
    //for these methods i will use my_age which is both an immutable variable and immutable string type - to avoid confusion
    //observe the fact that to access the result of immutable methods they have to be stored in a new variable
    //this is because this method make a clone of the value and perform manipulation and doesnt actually manipulate the original


    //.chars()-makes the string an iteration of each characters
    //please note: my_age.chars is ONLY an iteration and not an array-list-vector
    let mut i:i8 = 0;
    for letter in my_age.chars(){
        println!("{}. {}", i, letter);
        i+=1;
    }

    //.contains("text or ch")-returns a boolean after performing check
    let result = my_age.contains("Hello");
    println!("{}", result);
    
    //.find("text") or rfind("text") - returns substring index
    let char_index = my_age.find("18");
    //{:?} is used because the find method returns an Option<usize> type
    //{:?} should be used for complex or optional types - 
    println!("{:?}", char_index);  

    // there are alot more mutable and immutable string methods, please do research later

    //string concatenation
    //kindly note that string literals cannot be concatenated
    //e,g let s = "Hello" + &"world";
    //that is why they are converted to string objects first
    let s1 = "Ihemedo".to_string();
    let s2 = "Chinedu".to_string();
    //please note using + operator in rust, rust moves the ownership of s1 using the left-hand rule
    //it moves the ownership to the final string created, fullname
    //s1 can no longer be accessed individually after this, but s2 can still be used/borrowed unless in a case of s1 + s2 +s3, the trend should begin to make sense to you
    // let fullname = s1 + &s2;
    // println!("{}", fullname);
    
    //a much better way would be to use the format!() macro
    let fullname = format!("{} {}", s1, s2);
    println!("{}", fullname);

}