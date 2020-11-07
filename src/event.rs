use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use crate::common::Either;

pub type ID = usize;
struct EventBase<'a,T>{
    next_id : ID,
    slots : Vec<(ID,Box<dyn 'a + FnMut(&T)>)>
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
        let mut event_closure = event.clone();
        self.listen(move|x|{
            event_closure.emit(&f(x))
        });
        event
    }

    pub fn fold<U>(&mut self,value : U,f : impl 'a + Fn(&T,&U) -> U) -> Event<'a,U>
        where T : 'a,
              U : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
        let make_closure = || -> Box<dyn FnMut(&T)>{
            let mut v = value;
            Box::new(move |x|{
                v = f(x,&v);
                event_closure.emit(&v);
            })
        };
        self.listen(make_closure());
        event
    }

    pub fn filter(&mut self,f : impl 'a + Fn(&T) -> bool) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
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
        let mut event_comb_closure1 = event_comb.clone();
        self.listen(move|x|{
            event_comb_closure1.emit(&Either::Left(x.clone()));
        });
        let mut event_comb_closure2 = event_comb.clone();
        event_rhs.listen(move|x|{
            event_comb_closure2.emit(&Either::Right(x.clone()));
        });
        event_comb
    }

    pub fn drop(&mut self,count : usize) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
        let make_closure = move || -> Box<dyn FnMut(&T)> {
            let mut cnt = count;
            Box::new(move |x| {
                if cnt == 0 {
                    event_closure.emit(x);
                }else{
                    cnt -= 1;
                }
            })
        };
        self.listen(make_closure());
        event
    }

    pub fn take(&mut self,count : usize) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
        let make_closure = move || -> Box<dyn FnMut(&T)> {
            let mut cnt = count;
            Box::new(move |x| {
                if cnt > 0 {
                    cnt -= 1;
                    event_closure.emit(x);
                }
            })
        };
        self.listen(make_closure());
        event
    }

    pub fn until<U>(&mut self,event_rhs : &mut Event<'a,U>) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
        let is_stopped = Rc::new(RefCell::new(false));
        let is_stopped_for_self = is_stopped.clone();
        event_rhs.listen(move |_|{
            *is_stopped.borrow_mut().deref_mut() = true;
        });
        self.listen(move |x|{
            if !*is_stopped_for_self.borrow().deref() {
                event_closure.emit(x);
            }
        });
        event
    }

    pub fn start<U>(&mut self,event_rhs : &mut Event<'a,U>) -> Event<'a,T>
        where T : 'a{
        let event = Event::new();
        let mut event_closure = event.clone();
        let is_stopped = Rc::new(RefCell::new(true));
        let is_stopped_for_self = is_stopped.clone();
        event_rhs.listen(move |_|{
            *is_stopped.borrow_mut().deref_mut() = false;
        });
        self.listen(move |x|{
            if !*is_stopped_for_self.borrow().deref() {
                event_closure.emit(x);
            }
        });
        event
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

    pub fn listen(&mut self,f : impl 'a + FnMut(&T)) -> ID{
        let mut refc = self.base.deref().borrow_mut();
        let id = refc.next_id;
        refc.next_id += 1;
        refc.slots.push((id,Box::new(f)));
        id
    }

    pub fn emit(&mut self,data : &T){
        let slots = &mut self.base
            .deref()
            .borrow_mut()
            .slots;
        for (_,f) in slots.iter_mut(){
            f(data)
        }
    }
}

impl<'a,T,U> Event<'a,Either<T,U>>{
    pub fn split(&mut self) -> (Event<'a,T>,Event<'a,U>)
        where T : 'a ,
              U : 'a{
        let event_left = Event::new();
        let mut event_left_closure = event_left.clone();
        let event_right = Event::new();
        let mut event_right_closure = event_right.clone();
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
    let mut src = Event::<i32>::new();
    let mut sum = src
        .map(|x| x + 1)
        .fold(0,|x,y| {
            *x + *y
        });
    sum.listen(|x|println!("sum:{}",*x));
    src.emit(&1);
    src.emit(&1);
    src.emit(&1);
    src.emit(&4);

    let mut src2 = Event::new();
    let mut dp = src2.drop(3);
    let mut tk = src2.take(3);
    dp.listen(|x| println!("dropped : {}",x));
    tk.listen(|x| println!("took:{}",x));
    src2.emit(&1);
    src2.emit(&2);
    src2.emit(&3);
    src2.emit(&4);
    src2.emit(&5);

    let mut src3 = Event::new();
    let mut src4 = Event::new();
    let mut unt = src3.until(&mut src4);
    let mut stt = src3.start(&mut src4);
    unt.listen(|x| println!("until:{}",x));
    stt.listen(|x| println!("start:{}",x));
    src3.emit(&1);
    src3.emit(&2);
    src3.emit(&3);
    src4.emit(&1);
    src3.emit(&4);

    let mut src5 = Event::new();
    {
        let mut aa = src5.map(|x| *x);
        let id = aa.listen(|_| println!("aa"));
        src5.emit(&3);
        aa.unlisten(id);
    }
    src5.emit(&3);

}
