use rand::{distributions::Alphanumeric, Error, Rng};
use std::io; // 0.8
// pub struct Note {
//     content: String,
// }
// impl Note{
//     fn add_content(mut self, s: String){
//         self.content = s;
//     }
// }
pub struct Page {
    contents: String,
    id: String,
}
impl Page {
    fn get_new_id() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        s
    }
    pub fn new_page(content: String, id: String) -> Page {
        let mut new_content = String::new();
        let mut new_page = Page {
            contents: String::new(),
            id: String::new(),
        };
        match io::stdin().read_line(&mut new_content) {
            Ok(n) => {
                new_page.contents = new_content;
                new_page.id = Page::get_new_id();
                println!("{} bytes read", n);
                println!("{}", new_page.contents);
                println!("{}", new_page.id);
                new_page
            }

            Err(err) => new_page,
        }
    }
}

fn main() {
    let page1 = Page::new_page(String::from("test"),String::from("Test_ID"));
    let page_vec: Vec<Page> = Vec::new();
    // let mut new_content = String::new();
    // let mut new_page = Page {
    //     contents: Content {
    //         content: String::new(),
    //     },
    //     id: String::new(),
    // };
    // match io::stdin().read_line(&mut new_content) {
    //     Ok(n) => {
    //         new_page.contents.content = new_content;
    //         new_page.id = Page::get_new_id();
    //         println!("{} bytes read", n);
    //         println!("{}", new_page.contents.content);
    //         println!("{}", new_page.id);
    //     }
    //     Err(error) => println!("error: {error}"),
    // }
}
