fn main() {
    {
        use chapter17::{Screen, Button, SelectBox};

        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: 30,
                    height: 40,
                    label: String::from("my button")
                }),
                Box::new(SelectBox {
                    width: 70,
                    height: 35,
                    options: vec![
                        String::from("option1"),
                        String::from("option2"),
                    ]
                })
            ]
        };
    }

    {
        use chapter17::blog::Post;

        fn main() {
            let mut post = Post::new();
        
            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());
        
            post.request_review();
            assert_eq!("", post.content());
        
            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}

