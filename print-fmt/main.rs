// Learn how to format prints

fn main() {

    //primitive fmt
    println!("{}", "Print formatting is cool!");

    //Indexed fmt
    println!("{0}, {1}, {2}", "First", "Second", "Third");

    //named fmt
    println!("{subject} {verb} {noun}", subject="Joe", verb="eats", noun="bananas");

    //mixed fmt
    println!("{0} {named}", "first", named="second");

}
