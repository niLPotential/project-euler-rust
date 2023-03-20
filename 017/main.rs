fn main() {
    let mut count: u32 = 0;

    count += 11; // one thousand
    count += 11 * 100; // nine hundred
    count += 12 * 100; // eight hundred
    count += 12 * 100; // seven hundred
    count += 10 * 100; // six hundred
    count += 11 * 100; // five hundred
    count += 11 * 100; // four hundred
    count += 12 * 100; // three hundred
    count += 10 * 100; // two hundred
    count += 10 * 100; // one hundred

    count += 3 * 99 * 9; // and

    count += 6 * 10 * 10; // ninety
    count += 6 * 10 * 10; // eighty
    count += 7 * 10 * 10; // seventy
    count += 5 * 10 * 10; // sixty
    count += 5 * 10 * 10; // fifty
    count += 5 * 10 * 10; // forty
    count += 6 * 10 * 10; // thirty
    count += 6 * 10 * 10; // twenty

    count += 8 * 10; // nineteen
    count += 8 * 10; // eighteen
    count += 9 * 10; // seventeen
    count += 7 * 10; // sixteen
    count += 7 * 10; // fifteen
    count += 8 * 10; // fourteen
    count += 8 * 10; // thirteen
    count += 6 * 10; // twelve
    count += 6 * 10; // eleven
    count += 3 * 10; // ten

    count += 4 * 9 * 10; // nine
    count += 5 * 9 * 10; // eight
    count += 5 * 9 * 10; // seven
    count += 3 * 9 * 10; // six
    count += 4 * 9 * 10; // five
    count += 4 * 9 * 10; // four
    count += 5 * 9 * 10; // three
    count += 3 * 9 * 10; // two
    count += 3 * 9 * 10; // one

    println!("Problem 017: {count}");
}
