#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    // let mut score_iter = scores.iter();
    // let first = score_iter.next();

    // let mut score_iter = scores.iter_mut();
    // let first = score_iter.next();

    // let mut score_iter = scores.into_iter();
    // let first = score_iter.next();

    // for (team, score) in &scores {
    //     println!("{team} Got: {score} points");
    // }

    // for (team, score) in &mut scores {
    //     println!("{team} Got: {score} points");
    // }

    for (team, score) in scores {
        println!("{team} Got: {score} points");
    }
}

pub fn iterating_immutably() {
    // Make the code compile by filling in the `???`s
    // let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
    //
    // let mut my_iterable_fav_fruits = ???;   // Step 1
    //
    // assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // assert_eq!(my_iterable_fav_fruits.next(), ???);     // Step 2
    // assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // assert_eq!(my_iterable_fav_fruits.next(), ???);     // Step 3
    // assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // assert_eq!(my_iterable_fav_fruits.next(), ???);     // Step 4

    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);
}

pub fn iterating_mutably() {
    // Make the code compile by only modifying the loop.
    // let mut nums = [0, 1, 2, 3, 4];
    // let odd_nums = [1, 3, 5, 7, 9];
    // for num in nums.iter() {
    //     *num = 2 * *num + 1;
    // }
    // assert_eq!(nums, odd_nums)

    let mut nums = [0, 1, 2, 3, 4];
    let odd_nums = [1, 3, 5, 7, 9];
    for num in nums.iter_mut() {
        *num = 2 * *num + 1;
    }
    assert_eq!(nums, odd_nums)
}

pub fn hashmaps() {
    // Fix the code to make it compile.
    // use std::collections::HashMap;
    //
    // // marks scored out of 50
    // let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    // // convert marks into percentage
    // for (_, marks) in marks {
    //     *marks = (*marks * 100.0) / 50.0;
    // }
    // marks.for_each(|(student, marks)| println!("{student} scored {marks}%"));

    use std::collections::HashMap;

    // marks scored out of 50
    let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    // convert marks into percentage
    for (_, marks) in &mut marks {
        *marks = (*marks * 100.0) / 50.0;
    }
    marks
        .iter()
        .for_each(|(student, marks)| println!("{student} scored {marks}%"));
}
