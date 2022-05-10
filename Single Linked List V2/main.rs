#[derive(Clone)]
struct Node<T: Clone> {
    _data: T,
    _next: Option<Box<Node<T>>>
}
struct LinkedList<T: Clone> {
    _head: Option<Box<Node<T>>>,
    _tail: *mut Option<Box<Node<T>>>
}

impl<T: Clone> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList{_head: None, _tail: std::ptr::null_mut()}
    }

    fn add(&mut self, new_data: T) {
        match self._head.as_mut() {
            Some(node) => {
                let new_node = Box::<Node<T>>::new(Node{_data: new_data, _next: None});

                match node._next.as_mut() {
                    Some(_) => {
                        unsafe {
                            (*self._tail).as_mut().unwrap()._next = Some(new_node);
                            
                            self._tail = &mut (*self._tail).as_mut().unwrap()._next;
                        }
                    }

                    None => {
                        node._next = Some(new_node);
                        self._tail = &mut node._next;
                    }
                }
            }

            None => {
                self._head = Some(Box::<Node<T>>::new(Node{_data: new_data, _next: None}));
            }
        }
    }

    fn get(&mut self) -> Option<T>{
        match self._head.as_ref() {
            Some(_) => {
                let data = self._head.as_ref().unwrap()._data.clone();
                self._head = self._head.as_ref().unwrap()._next.clone();

                Some(data)
            }

            None => {
                None
            }
        }        
    }
}

fn main() {
    let mut list: LinkedList<i64> = LinkedList::new();
    
    for i in 1..10 {
        println!("Input {}", i);

        list.add(i);
    }
    
    loop {
        let val = list.get();
        
        if val != None {
            println!("{}", val.unwrap());
        } else {
            println!("List is empty");

            break;
        }
    }
}
