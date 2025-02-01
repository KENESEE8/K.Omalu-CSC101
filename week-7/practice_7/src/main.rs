fn main(){
    //array with data type (explicit integer datatype)
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with datatype");
    println!("array is {:?}",arr1);
    println!("array siz is :{}",arr1.len());

        //array without data type (implicit float datatype)
        let arr2 = [10.4,20.7,30.4,40.9,51.1];
        println!("\nArray with datatype");
        println!("array is {:?}",arr2);
        println!("array siz is :{}",arr2.len());

            //array with default values that creates and
            //initializes all its elements with a default value of -1
    let arr3:[i32;8] = [-1;8];
    println!("\nArray with datatype");
    println!("array is {:?}",arr3);
    println!("array siz is :{}",arr3.len());

}
