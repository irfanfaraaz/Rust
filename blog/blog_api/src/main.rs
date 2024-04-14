use blog_shared::Post;

fn main() {
    let post = Post::new("Irfan".to_owned(), "This is my post".to_string());
    println!("{:?}", post);
}
