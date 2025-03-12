fn compound_data_type(){
    
    // the compound data types in rust are arrays, tuples, slices and strings (slice strings)

    // Arrays are collection of items of homogenous data type
    // Arrays are immutable
    // Arrays are of fixed length


    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let alphabets: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];

    println!("\n\t--------Compound data types--------");
    println!("numbers array: {:?}", numbers);
    println!("alphabets array: {:?}", alphabets);
    println!("fruits array: {:?}", fruits);
    println!("fruits array 1: {:?}", fruits[0]);
    println!("fruits array 2: {:?}", fruits[1]);
    println!("fruits array 3: {:?}", fruits[2]);


    // Tuples
    // Tuples are collection of items of hetrogeneous data types(can be of different data types)
    // Tuples are immutable and can be of any length

    let tuple: (i32, char, &str) = (1, 'a', "apple");
    let tuple1: (i32, char, &str) = (2, 'b', "banana");
    let tuple2: (i32, char, &str) = (3, 'c', "cherry");
    let tuple3= (3, 4, 5, "asd", 2.4);

    println!("\n\t--------Compound data types--------");
    println!("tuple: {:?}", tuple);
    println!("tuple1: {:?}", tuple1);
    println!("tuple2: {:?}", tuple2);
    println!("tuple3: {:?}", tuple3);
}