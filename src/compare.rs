 pub fn compareString(x:&str,y:&str)->bool{
    let lenx=x.len();
    let leny=y.len();
    let mut i=0;
    loop {
        if i==leny{
            break true;
        }
        if x.as_bytes()[i]<y.as_bytes()[i]{}
        else if x.as_bytes()[i]>y.as_bytes()[i]{
            break false;
        }
        i=i+1;
        if i==lenx{
            if i<leny{
                break false;
            }
            else {
                break true;
            }
        }
    }
}