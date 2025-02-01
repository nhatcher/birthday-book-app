import json
import os

from pybook import from_json_string, create, read_from_file;

class Friend(object):
    name: str
    email: str
    last_name: str
    birthday: str | None

def run_tests():
    book = create("Fermi")

    friend = Friend()
    friend.name = "Idling"
    friend.last_name ="Raymund"
    friend.email = "i.raymund@equalto.com"
    # Happy birthday dude!
    friend.birthday = "27/03"
    book.add_friend(friend)

    book_b = json.loads(book.to_json_string())
    assert book_b["owner"], "Fermi"
    assert len(book_b["friends"]), 1

    book.write_to_file("temp.json")

    book_c = read_from_file("temp.json")

    first_friend = book_c.get_first_friend_with_last_name("Raymund")
    assert first_friend.name, "Idling"

    with open("temp.json", "r") as f:
        book_d = from_json_string(str(f.read()))
        assert book_d.get_owner(), "Fermi"

    os.remove("temp.json")



if __name__ == "__main__":
    run_tests()
    print("All tests pass.")

