use std::{marker::PhantomData, collections::HashMap, any::{TypeId, Any}, fmt::Display, rc::Rc};

type CollectionType = HashMap<TypeId, Rc<dyn Any>>;




pub struct ArgsCollection {
    pub args: CollectionType,
}

pub trait GlobalTrait: Any + Sized {
    fn to_any(self) -> Box<dyn Any> { Box::new(self) }
}


impl ArgsCollection {
    pub fn init() -> Self {
        ArgsCollection { args: HashMap::new() }
    }

    pub fn add_param<T: SystemParam + 'static>(self: &mut Self, val: T) {
        self.args.insert(TypeId::of::<T>(), Rc::new(val.get_inner()));
    }

    pub fn add_global(self: &mut Self, global: Rc<dyn Any>) {


        self.args.insert((*global).type_id(), global);
    }
}

pub struct FunctionStruct<Input, F> {
    f: F,
    marker: PhantomData<Input>,
}

pub trait System {
    fn call_system(&mut self, args: &mut HashMap<TypeId, Rc<dyn Any>>);
}

pub trait IntoSystem<Input, F> {
    type System: System;
    fn into_system(self) -> Self::System;
}

// impl<F, T1: SystemParam, T2: SystemParam> System for FunctionStruct<(T1, T2,), F> 
// where for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::FnParamType<'b>, <T2 as SystemParam>::FnParamType<'b>)
// {
//     fn call_system (&mut self, args: &mut HashMap<TypeId, Box<dyn Any>>) {
//         let T1 = T1::get_value(args);
    
//         let T2 = T2::get_value(args);

//         fn call_inner<F: FnMut(T1, T2), T1, T2>(mut f: F, T1: T1, T2: T2) {

//         }
    
//         call_inner(&mut self.f, T1, T2);
//     }
// }

// impl<F: FnMut(T1, T2,), T1: SystemParam, T2: SystemParam> IntoSystem<(T1, T2,), F> for F 
// where for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::FnParamType<'b>, <T2 as SystemParam>::FnParamType<'b>)
// {
//     type System = FunctionStruct<(T1, T2,), F>;    
    
//     fn into_system (self) -> Self::System {
    
//         FunctionStruct {
//             f: self,
//             marker: PhantomData::default()
//         }
//     }
// }

impl<F, T1: SystemParam,> System for FunctionStruct<(T1,), F> 
where for<'a, 'b> &'a mut F: FnMut(T1,) + FnMut(<T1 as SystemParam>::FnParamType<'b>)
{
    fn call_system (&mut self, args: &mut HashMap<TypeId, Rc<dyn Any>>) {
        let T1 = T1::get_value(args);
    

        fn call_inner<F: FnMut(T1), T1>(mut f: F, T1: T1) {
            f(T1);
        }
    
        call_inner(&mut self.f, T1,);
    }
}

impl<F: FnMut(T1,), T1: SystemParam,> IntoSystem<(T1, ), F> for F 
where for<'a, 'b> &'a mut F: FnMut(T1,) + FnMut(<T1 as SystemParam>::FnParamType<'b>)
{
    type System = FunctionStruct<(T1,), F>;    
    
    fn into_system (self) -> Self::System {
    
        FunctionStruct {
            f: self,
            marker: PhantomData::default()
        }
    }
}

// impl<F, T1: SystemParam, T2: SystemParam> System for FunctionStruct<(T1, T2), F> 
// where for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::FnParamType<'b>, <T2 as SystemParam>::FnParamType<'b>)
// {
//     fn call_system (&mut self, args: &mut HashMap<TypeId, Box<dyn Any>>) {
//         let T1 = T1::get_value(args);

//         let T2 = T2::get_value(args);
    

//         fn call_inner<F: FnMut(T1, T2), T1, T2>(mut f: F, T1: T1, T2: T2) {
//             f(T1, T2);
//         }
    
//         call_inner(&mut self.f, T1, T2);
//     }
// }

// impl<F: FnMut(T1, T2), T1: SystemParam, T2: SystemParam> IntoSystem<(T1, T2), F> for F 
// where for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::FnParamType<'b>, <T2 as SystemParam>::FnParamType<'b>)
// {
//     type System = FunctionStruct<(T1, T2), F>;    
    
//     fn into_system (self) -> Self::System {
    
//         FunctionStruct {
//             f: self,
//             marker: PhantomData::default()
//         }
//     }
// }


pub trait SystemParam {
    type FnParamType<'new>;

    type InnerType;

    fn get_value<'r>(args: &'r mut HashMap<TypeId, Rc<dyn Any>>) -> Self::FnParamType<'r>;

    fn get_inner(self) -> Self::InnerType;

}

pub struct Req<T> {
    pub inner: T
}

pub struct Res<T> {
    pub inner: T
}

#[derive(Clone, Copy)]
pub struct Global<T> {
    pub inner: T
}


impl<T: 'static> GlobalTrait for Global<T> {}

impl<'a, T: Any + Copy> SystemParam for Global<T> {
    type FnParamType<'new> = Global<T>;

    type InnerType = T;

    fn get_value<'r>(args: &'r mut HashMap<TypeId, Rc<dyn Any>>) -> Self::FnParamType<'r> {

        
        dbg!("KEYS {}", args.keys());

        dbg!("IN ACTUAL {}", &TypeId::of::<Self::FnParamType::<'static>>());

        return *args.remove(&TypeId::of::<Self::FnParamType::<'static>>()).unwrap().downcast::<Self::FnParamType<'r>>().unwrap()
    }

    fn get_inner(self) -> Self::InnerType {
        return self.inner
    }

}

impl<T: Any + Copy> SystemParam for Res<T> {
    type FnParamType<'new> = Res<T>;

    type InnerType = T;

    fn get_value<'r>(args: &'r mut HashMap<TypeId, Rc<dyn Any>>) -> Self::FnParamType<'r> {
        return Res {inner: *args.remove(&TypeId::of::<Self>()).unwrap().downcast::<T>().unwrap()};
    }

    fn get_inner(self) -> Self::InnerType {
        return self.inner
    }

    
}

impl<T: Any + Copy> SystemParam for Req<T> {  
    type FnParamType<'new> = Req<T>;

    type InnerType = T;

    fn get_value<'r> (args: &'r mut HashMap<TypeId, Rc<dyn Any>>) -> Self::FnParamType<'r> {
        return Req {inner: *args.remove(&TypeId::of::<Self>()).unwrap().downcast::<T>().unwrap()};
    }

    fn get_inner(self) -> Self::InnerType {
        return self.inner
    }

    
}

fn system(res: Res<i32>, req: Req<String>) {
    println!("req {}, res {}", req.inner, res.inner);
}


// fn main() {
//     let mut args = ArgsCollection::init();

//     args.add_param(Req::<String> {
//         inner: "Hey".to_string()
//     });

//     args.add_param(Res::<i32> {
//         inner: 12
//     });

//     system.into_system().call_system(&mut args.args);

// }