mod linked_list;
mod bubble_sort;

// contains bubble sort and a really bad linked_list implementation

fn main() {
    println!("Hello, world!");

    let mut vec1 = vec! {1, 2, 3, 2, 1};
    bubble_sort::sort(&mut vec1);

    for x in vec1 {
        println!("{:?}", x)
    }

    run_list();
}

fn run_list() {
    let mut list = linked_list::MyList::create();

    println!("{:?}", list);

    list.add(5);
    println!("{:?}", list);

    list.add(5);
    println!("{:?}", list);

    list.add(5);
    println!("{:?}", list);
}
