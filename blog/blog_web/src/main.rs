use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on web".to_string(),
        "This is my post on web".to_string(),
    );
    println!("{:?}", post);
}
