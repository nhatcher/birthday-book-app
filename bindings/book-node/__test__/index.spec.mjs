import test from 'ava';

import { NodeBook } from '../index.js';
import * as fs from 'fs';

test('can create book', (t) => {
  const book = new NodeBook("Fermi");
  book.addFriend({
    name: "Idling",
    lastName: "Raymund",
    email: "i.raymund@equalto.com",
    birthday: "27/03"
  });

  const book_b = JSON.parse(book.toJSON());
  t.is(book_b.owner, "Fermi");
  t.is(Object.keys(book_b.friends).length, 1);

  book.writeToFile("temp.json");

  const book_c = NodeBook.readFromFile("temp.json");

  const firstFriend = book_c.getFirstFriendWithLastName("Raymund");
  t.is(firstFriend.name, "Idling");

  const book_d = NodeBook.fromJSON(fs.readFileSync('temp.json', 'utf8'));

  t.is(book_d.getOwner(), "Fermi" );

  fs.unlinkSync("temp.json");
})
