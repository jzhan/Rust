struct Node<T> {
    data: T,
    _next: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    _head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T>{
        LinkedList{_head: None}
    }

    fn add(&mut self, _new_data: T) {
        match self._head.as_ref() {
            Some(_) => {
                let mut temp = &mut self._head;

                loop {
                    match temp.as_ref().unwrap()._next.as_ref() {
                        Some(_) => {
                            temp = &mut temp.as_mut().unwrap()._next;
                        }

                        None => {
                            temp.as_mut().unwrap()._next = Some(Box::<Node<T>>::new(Node{data: _new_data, _next: None}));

                            break;
                        }
                    }
                }
            }

            None => {
                self._head = Some(Box::<Node<T>>::new(Node{data: _new_data, _next: None}));
            }
        }
    }

    fn get(mut self) -> (Option<T>, LinkedList<T>){
        match self._head.as_ref() {
            Some(_) => {
                let data = self._head.unwrap();
                self._head = data._next;
                (Some(data.data), self)
            }

            None => {
                (None, self)
            }
        }        
    }
}

fn main() {
    let mut _list: LinkedList<i32> = LinkedList::new();

    _list.add(3);
    _list.add(4);

    {
        let mut val;

        (val, _list) = _list.get();

        if val != None {
            println!("{}", val.unwrap());
        }
        
        // _list.add(5);

        (val, _list) = _list.get();

        if val != None {
            println!("{}", val.unwrap());
        }

        (val, _list) = _list.get();

        if val != None {
            println!("{}", val.unwrap());
        } else {
            println!("_list is empty");
        }
    }
}
