mod domain;

use domain::blog::Blog;

fn main() {
    let mut blog = Blog::new("title".to_string(), "content".to_string(), "phodal".to_string());
    blog.update_title("title".to_string());
    println!("{:?}", blog.title);
}
