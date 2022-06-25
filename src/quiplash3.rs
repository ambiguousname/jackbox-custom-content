const PATH : String = String::from("./Quiplash3/content/");

mod quiplash3 {
    pub struct round1_question;

    fn load_round_question(path : String) -> Vector<&Content> {
        let list_path = Path::new(path + ".jet");
        
    }

    impl ContentCategory for round1_question {
        const LOCAL_PATH : String = PATH + String::from("/Round1Question/");
        fn load_content() -> Vector<&Content> {
            load_round_question(LOCAL_PATH)
        } 
    }
}