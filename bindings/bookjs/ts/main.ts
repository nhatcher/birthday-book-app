import init, { WasmBook } from '../pkg/bookjs.js';

await init();

function run() {
    const book = new WasmBook("Fermi");
    book.addFriend({
        name: "Albert",
        last_name: "Einstein",
        email: "einstein@theuniverse.today",
        birthday: null
    });

    // George Uhlenbeck together with Goudsmit introduced the electron spin
    book.addFriend({
        name: "George",
        last_name: "Uhlenbeck",
        email: "g.uhlenbeck@theuniverse.today",
        birthday: null
    });

    // Karen Uhlenbeck is one of the founders of geometric analysis
    // Her book on instantons and four manifolds with Daniel Freed in a gem in modern mathematics.
    book.addFriend({
        name: "Karen",
        last_name: "Uhlenbeck",
        email:"k.uhlenbeck@theuniverse.today",
        birthday: null
    });

    const uhlenbeck_count = book.getFriendCountWithLastName("Uhlenbeck");
    console.log("Friends by name Uhlenbeck: ", uhlenbeck_count);
    console.assert(uhlenbeck_count == 2);

    console.log("First of the Uhlenbecks:", book.getFirstFriendWithLastName("Uhlenbeck"))
    console.log(JSON.parse(book.toJSON()));

    try {
        book.addFriend({
            name: "George",
            last_name: "Uhlenbeck",
            email: "g.uhlenbeck@theuniverse.today",
            birthday: null
        });
        console.error("Exception should have been raised!");
    } catch (e) {
        console.log("Exception raised as expected: ", e);
    }
}


export default run;