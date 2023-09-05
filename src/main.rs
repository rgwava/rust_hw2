mod compare;
mod buffer_t;
use crate::buffer_t::buffer;
use crate::compare::compareString;
fn main() {
    //直接cargo run即可
    println!("iter-closure---------------");
    let chr=vec!['a','b','c','d','e'];
    let chr_iter=chr.iter().map(|x|(*x as u8 +1)as char);
    for i in chr_iter{
        println!("{}",i);
    }
    println!("buffer----------------");
    let v=vec![0,1,2,3,4,5];
    let buf = buffer::buffer(v);
    println!("{}",buf.sum());


    println!("compare----------------");
    println!("aaaa and aaaa {}",compareString("aaaa","aaaa"));
    println!("aaa and aaaa {}",compareString("aaa","aaaa"));
    println!("aaaa and aaa {}",compareString("aaaa","aaa"));
    println!("abcd and aaaa {}",compareString("abcd","aaaa"));
    println!("aaaa and abcd {}",compareString("aaaa","abcd"));
}
