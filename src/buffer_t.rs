pub struct buffer<T>{
    v:Vec<T>,
}
impl<T: std::ops::Add<Output = T> + Default + Clone> buffer<T>{
    pub fn sum(&self)->T{
        let mut s=T::default();
        for i in &self.v{
            s=s+i.clone();
        }
        s
    }
    pub fn buffer(data:Vec<T>)->Self{
        buffer{v:data}
    }
}