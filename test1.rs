fn main(){
    
    println!("Hello World1");

    println!("{}", "Hello World2");

    println!("{} {}", "Hello", "World3");

    println!("{}", concat!("Hello", "World4"));

    println!("{}", ["Hello"," ","World5"].concat());

    println!("{}", ["Hello", "World6"].join(" "));

    println!("{}", format!("{} {}", "Hello", "World7"));

    println!("{}", "Hello".to_owned()+ " "+ "World8");


}