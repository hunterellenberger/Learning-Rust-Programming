fn main() {
    let foo = String::from("hello");
    let bar = String::from("world");

    let user = build_user(foo, bar);
    println!("{}, {}!", user.name, user.foo);
}

struct User{
    name: String,
    foo: String,
}

fn build_user(name: String, foo: String) -> User{
    User {name: name, foo: foo}
}
