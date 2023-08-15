use num::bigint::*;

fn fib(n: i32) -> BigInt {
    let mut a = 1.to_bigint().unwrap();
    let mut b = 0.to_bigint().unwrap();
    let mut p = 0.to_bigint().unwrap();
    let mut q = 1.to_bigint().unwrap();
    let sign = if n<0 && (n.abs()&1==0) {-1} else {1};
    let mut n = n.abs();
    loop {
        if n==0 {
            return b*sign;
        }
        else if n&1==0 {
            (p,q) = (p.clone()*p.clone()+q.clone()*q.clone(),q.clone()*q.clone()+2*p.clone()*q.clone());
            n /= 2;
        }
        else {
            (a,b) = (b.clone()*q.clone()+a.clone()*q.clone()+a.clone()*p.clone(),b*p.clone()+a.clone()*q.clone());
            n -= 1;
        }
    }
} 
   








fn main() {
    let a = 3;
    let b = 4;
    let c = 5;
    let d = 6;
    let much = 1_000_000;
    println!("{:?}",fib(a));
    println!("{:?}",fib(b));
    println!("{:?}",fib(c));
    println!("{:?}",fib(d));
    println!("{:?}",fib(much));



}
