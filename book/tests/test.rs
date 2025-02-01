use book::{Book, Friend};

#[test]
fn run_tests() {
    let mut book = Book::new("Fermi");
    book.add_friend(Friend {
        name: "Idling".to_string(),
        email: "i.raymund@equalto.com".to_string(),
        last_name: "Raymund".to_string(),
        birthday: Some("27/03".to_string()),
    })
    .unwrap();

    let book_b: Book = serde_json::from_str(&book.to_json_string().unwrap()).unwrap();

    assert_eq!(book_b.owner(), *"Fermi");
}
