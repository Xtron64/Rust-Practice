fn hello(name: String) -> String{
    return "Hello".to_string() + &name.to_string();
}

fn main(){
    hello("Xtron64".to_string());
}
