pub mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList = LinkedList::new();

    list.add_first(3);
    list.add_first(4);

}
