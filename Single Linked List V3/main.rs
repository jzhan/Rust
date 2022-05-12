#[derive(Clone)]
struct Node<T: Clone> {
    _data: T,
    _next: *mut Node<T>
}
struct LinkedList<T: Clone> {
    _head: *mut Node<T>,
    _tail: *mut Node<T>
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList{
            _head: std::ptr::null_mut(), 
            _tail: std::ptr::null_mut()
        }
    }

    fn add(&mut self, new_data: T) {
        let new_node = Box::new(
            Node{
                _data: new_data, 
                _next: std::ptr::null_mut()
            });

        if self._head == std::ptr::null_mut() {
            self._head = Box::into_raw(new_node);
            self._tail = self._head;
        } else {
            unsafe {
                (*self._tail)._next = Box::into_raw(new_node);
                self._tail = (*self._tail)._next;
            }
        }
    }

    fn get(&mut self) -> Option<T> {
        if self._head == std::ptr::null_mut() {
            None
        } else {
            unsafe {
                let data = (*self._head)._data.clone();
                let temp = self._head;
                self._head = (*self._head)._next;

                if self._head == std::ptr::null_mut() {
                    self._tail = std::ptr::null_mut();
                }
                
                drop(Box::from_raw(temp));
                
                Some(data)
            }
        }
    }
}

// Destructor
impl <T: Clone> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            loop {
                if self._head == std::ptr::null_mut() {
                    self._tail = std::ptr::null_mut();

                    break;
                }

                let temp = self._head;
                self._head = (*temp)._next;

                drop(Box::from_raw(temp));
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    
    for i in 1..10 {
        println!("Input {}", i);

        list.add(i);
    }

    println!();

    for _i in 1..5 {
        let val = list.get();
        
        if val != None {
            println!("{}", val.unwrap());
        } else {
            println!("List is empty");

            break;
        }
    }

    let list = 10;

    println!("new: {}", list);

    // for i in 11..15 {
    //     println!("Input {}", i);

    //     list.add(i);
    // }
    
    // loop {
    //     let val = list.get();
        
    //     if val != None {
    //         println!("{}", val.unwrap());
    //     } else {
    //         println!("List is empty");

    //         break;
    //     }
    // }
}
