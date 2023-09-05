
struct Buffer<T>{
    buf:Vec<T>,
}
impl<T: std::ops::Add<Output = T> + Default + Clone> Buffer<T> {
    fn sum(&self) -> T {
        let mut sum = T::default();
        for item in &self.buf {
            sum = sum + (*item).clone();
        }
        sum
    }

    fn buffer(data: Vec<T>) -> Self {
        Buffer { buf: data }
    }
}


fn main() { 
    
}


fn compareString(x: &str, y: &str) -> bool {
    
    let xx : Vec<char> = x.chars().collect();
    let yy : Vec<char>= y.chars().collect();

    let min_len = if xx.len() > yy.len(){
        xx.len()
    }else {
        yy.len()
    };

    for i in 0..(min_len - 1){
        if xx[i] < yy [i]{
            return  false;
        }
    }

    if xx.len() >= min_len{
        return  true;
    }
    false
}

fn new_vec( a : &Vec<char>) -> Vec<char> {
    a.iter().map(|c| (*c as u8 + 1 ) as char).collect()
}


#[cfg(test)]
mod tests {
    use crate::{Buffer, compareString, new_vec};

    #[test]
    fn exercises1(){
        let buffer:Buffer<i32> = Buffer::buffer(vec![1, 2, 3, 4, 5]);
        let total = buffer.sum();
        assert_eq!(total,15);

        let buffer: Buffer<i32> = Buffer { buf: Vec::new() };
        let total = buffer.sum();
        assert_eq!(total,0);
    }
    #[test]
    fn exercises2(){
        let s1:&str = "114514";
        let s2:&str = "1919810";

        assert_eq!(compareString(s1,s2),false);
    }
    #[test]
    fn exercises3(){
        let v1 = vec!['a','b','c','d','e'];
        let v2 = new_vec(&v1);
        assert_eq!(v2,vec!['b','c','d','e','f']);
    }
}