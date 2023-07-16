use std::{marker::PhantomData, collections::HashMap, any::{TypeId, Any}};

pub struct ArgsCollection {
    pub args: HashMap<TypeId, Box<dyn Any>>
}

impl ArgsCollection {
    pub fn init() -> Self {
        ArgsCollection { args: HashMap::new() }
    }

    pub fn add_param<T: SystemParam + 'static>(self: &mut Self, val: T) {
        self.args.insert(TypeId::of::<T>(), Box::new(val.cast_val()));
    }
}

pub struct FunctionStruct<Input, F> {
    f: F,
    marker: PhantomData<Input>
}

pub trait System {
    fn call_system(&mut self, args: &mut HashMap<TypeId, Box<dyn Any>>);
}

pub trait IntoSystem<Input, F> {
    type System: System;
    fn into_system(self) -> Self::System;
}

impl<F: FnMut(T1, T2), T1: SystemParam<FnParamType = T1>, T2: SystemParam<FnParamType = T2> > System for FunctionStruct<(T1, T2,), F> {
    fn call_system (&mut self, args: &mut HashMap<TypeId, Box<dyn Any>>) {
        let T1 = T1::get_value(args);
    
        let T2 = T2::get_value(args);
    
    
        (self.f)(T1, T2);
    }
}

impl<F: FnMut(T1, T2,), T1: SystemParam<FnParamType = T1>, T2: SystemParam<FnParamType = T2> > IntoSystem<(T1, T2,), F> for F {
    type System = FunctionStruct<(T1, T2,), F>;    
    
    fn into_system (self) -> Self::System {
    
        FunctionStruct {
            f: self,
            marker: PhantomData::default()
        }
    }
}


pub trait SystemParam {
    type FnParamType;

    type InnerType;

    fn get_value(args: &mut HashMap<TypeId, Box<dyn Any>>) -> Self::FnParamType;

    fn cast_val(self,) -> Self::InnerType;
}

pub struct Req<T> {
    pub inner: T
}

pub struct Res<T> {
    pub inner: T
}

impl<T: Any> SystemParam for Res<T> {
    type FnParamType = Res<T>;

    type InnerType = T;

    fn get_value(args: &mut HashMap<TypeId, Box<dyn Any>>) -> Self::FnParamType {
        return Res {inner: *args.remove(&TypeId::of::<Self::FnParamType>()).unwrap().downcast::<T>().unwrap()};
    }

    fn cast_val(self,) -> Self::InnerType {
        self.inner
    }
}

impl<T: Any> SystemParam for Req<T> {  
    type FnParamType = Req<T>;

    type InnerType = T;

    fn get_value(args: &mut HashMap<TypeId, Box<dyn Any>>) -> Self::FnParamType {
        return Req {inner: *args.remove(&TypeId::of::<Self::FnParamType>()).unwrap().downcast::<T>().unwrap()};
    }

    fn cast_val(self,) -> Self::InnerType {
        self.inner
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