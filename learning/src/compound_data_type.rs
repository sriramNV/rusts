fn compound_data_type(){
    
    // the compound data types in rust are arrays, tuples, slices and strings (slice strings)

    // Arrays are collection of items of homogenous data type

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let alphabets: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    println!("numbers: {:?}", numbers);
    println!("alphabets: {:?}", alphabets);
}