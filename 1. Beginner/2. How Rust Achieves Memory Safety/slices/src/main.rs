fn main() {
    let tweet = String::from("I am learning Rust! This is my tweet.");
    let trimmed_tweet: &str = trim_tweet(&tweet); // string slice (reference to a part of a string

    let tweet2 = "I am learning Rust! This is my tweet.";
    let trimmed_tweet2: &str = trim_tweet(tweet2); // string slice (reference to a part of a string
    println!("Tweet: {}", tweet);

    println!("Trimmed tweet: {trimmed_tweet}");

    println!("Trimmed tweet2: {trimmed_tweet2}");

    let a= [1,2,3,4,5];
    let a_slice = &a[1..3];
    println!("a_slice: {:?}", a_slice);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[0..10]//string slice
}
