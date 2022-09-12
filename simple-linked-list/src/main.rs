use simple_linked_list::*;

fn main() {
    println!("Simple linked list");
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);

    println!("length {:?}", list.len());
    println!("popped value {:?}", list.pop());
    println!("popped value {:?}", list.pop()); 

}
