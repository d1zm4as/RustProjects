fn main() {
    /*
     
      Macros são com funções mas eles se expandem em código adicional
     */
    // println "print" informação na tela, muito util para debugar
    let num:i32 =7;
    println!("Hello, world!");
    // {:?} diz pro rust que o print é para fins de debug
    println!("{:?}",num);
    println!("{:?} {:?}",num,num);
    println!("Zone {:?}",num);
    println!("{num:?}");
    println!("{num}");// esse print já é destinado ao usuário


    

}
