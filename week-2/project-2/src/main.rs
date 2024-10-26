fn main() {
    let T1 =450000;
    let M1 =1500000;
    let H1 =750000;
    let D1 =2850000;
    let A1 =250000;
    let T2 =2;
    let M2 =1;
    let H2 =3;
    let D2 =3;
    let A2 =1;
    //  sum
    let s = T1+M1+H1+D1+A1;
    println!("s  is : {}" ,s);
    // average
    let a = s/T2+M2+H2+D2+A2;
    println!("a is : {}" ,a);
}
