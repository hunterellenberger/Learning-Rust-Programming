fn main() {
    let mut s = String::from("testing");

    change(&mut s);
    change(&mut s);
    println!("{s}");
}

fn change(s: &mut String){
    s.push_str(" test");
}
