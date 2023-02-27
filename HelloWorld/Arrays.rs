fn main(){
    let arr: [u8; 5] = [1,2,3,4,5];
    println!("{:?} ---> {}", arr, arr.len());

    // Unlike python we can't append elements to the list.
    // we need to create one duplicate list and add element to it.
    let mut array = ["AIE","CSE","ECE","EEE","ELC"];
    println!("{:?}",array);
    println!("Array index 4 --> {}",array[3]);
    array[3] = "BCE";
    println!("Array index 4 --> {}",array[3]);
}