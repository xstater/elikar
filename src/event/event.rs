use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use crate::common::Either;

pub type ID = usize;
struct EventBase<'a,T>{
    next_id : ID,
    slots : Vec<(ID,Box<dyn 'a + Fn(&T)>)>
}

pub struct Event<'a,T>{
    base : Rc<RefCell<EventBase<'a,T>>>
}

impl<'a,T> Clone for Event<'a,T>{
    fn clone(&self) -> Self {
        Event{
            base : self.base.clone()
        }
    }
}

impl<'a,T> Event<'a,T>{
    pub fn new() -> Event<'a,T>{
        Event{
            base : Rc::new(RefCell::new(
                EventBase{
                    next_id : 0,
                    slots : vec![]
                }
            ))
        }
    }

    pub fn map<U : 'a>(&mut self,f : impl 'a + Fn(&T) -> U) -> Event<'a,U>{
        let event = Event::new();
        let event_closure = event.clone();
        self.listen(move|x|{
            event_closure.emit(&f(x))
        });
        event
    }

    pub fn filter(&mut self,f : impl 'a + Fn(&T) -> bool) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let event_closure = event.clone();
        self.listen(move|x|{
            if f(x) {
                event_closure.emit(x)
            }
        });
        event
    }

    pub fn merge<U>(&mut self,event_rhs : &mut Event<'a,U>) -> Event<'a,Either<T,U>>
        where T : 'a + Clone,
              U : 'a + Clone{
        let event_comb : Event<Either<T,U>> = Event::new();
        let event_comb_closure1 = event_comb.clone();
        self.listen(move|x|{
            event_comb_closure1.emit(&Either::Left(x.clone()));
        });
        let event_comb_closure2 = event_comb.clone();
        event_rhs.listen(move|x|{
           event_comb_closure2.emit(&Either::Right(x.clone()));
        });
        event_comb
    }

    pub fn unlisten(&mut self,id : ID){
        let slots = &mut self.base.deref().borrow_mut().slots;
        for (i,(slot_id,_)) in slots.iter().enumerate(){
            if *slot_id == id {
                slots.remove(i);
                return;
            }
        }
    }

    pub fn listen(&mut self,f : impl 'a + Fn(&T)) -> ID{
        let mut refc = self.base.deref().borrow_mut();
        let id = refc.next_id;
        refc.next_id += 1;
        refc.slots.push((id,Box::new(f)));
        id
    }

    pub fn emit(&self,data : &T){
        let slots = &self.base
            .deref()
            .borrow()
            .slots;
        for (_,f) in slots{
            f(data)
        }
    }
}

impl<'a,T,U> Event<'a,Either<T,U>>{
    pub fn split(&mut self) -> (Event<'a,T>,Event<'a,U>)
        where T : 'a ,
              U : 'a{
        let event_left = Event::new();
        let event_left_closure = event_left.clone();
        let event_right = Event::new();
        let event_right_closure = event_right.clone();
        self.listen(move|x| match x {
            Either::Left(left) => {
                event_left_closure.emit(left);
            }
            Either::Right(right) => {
                event_right_closure.emit(right);
            }
        });
        (event_left,event_right)
    }
}

#[test]
fn target_sem(){
    let mut e1 = Event::new();
    let mut e2 = Event::new();
    let mut comb = e1.merge(&mut e2);
    let id = comb.listen(|x| println!("{:?}",*x));
    e1.emit(&1);
    e2.emit(&"asd".to_string());
    let (mut e11,mut e22) = comb.split();
    e11.listen(|x|println!("{}",x));
    e22.listen(|x|println!("{}",x));
    comb.unlisten(id);
    e1.emit(&2);
    e2.emit(&"asdsad".to_string());
}