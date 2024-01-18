// what is this ensuring:
// 1) Parent lives longer than Child.
// 2) Parent doesn't move while inside.

use std::{marker::PhantomPinned, pin::Pin};

/// LTPC: LifeTime Parent Child.
/// Get child that holds lifetime reference to parent into a container struct without a lifetime, while making sure child can't outlive parent.
/// This is a bad idea, should get rid of this. https://users.rust-lang.org/t/why-is-the-mutable-reference-not-being-dropped-after-the-function-call/105308

pub struct Ltpc<P, C>
where
    for<'a> C: 'a,
{
    raw: Pin<Box<LtpcRaw<P, C>>>,
}

struct LtpcRaw<P, C> {
    // parent is never None.
    parent: Option<P>,
    child: Option<C>,
    _pin: PhantomPinned,
}

impl<P, C> Ltpc<P, C>
where
    for<'a> C: 'a,
{
    pub fn new(parent: P) -> Self {
        Self {
            raw: Box::pin(LtpcRaw {
                parent: Some(parent),
                child: None,
                _pin: PhantomPinned,
            }),
        }
    }

    pub fn parent(&self) -> &P {
        self.raw.parent.as_ref().unwrap()
    }

    pub fn parent_mut(&mut self) -> &mut P {
        let raw = self._get_raw_mut();
        raw.parent.as_mut().unwrap()
    }

    pub fn child<'a>(&'a self) -> Option<&'a C> {
        self.raw.child.as_ref()
    }

    pub fn child_mut(&mut self) -> Option<&mut C> {
        let raw = self._get_raw_mut();
        let child = raw.child.as_mut();
        let child: Option<&mut C> = unsafe { std::mem::transmute(child) };
        child
    }

    fn _get_raw_mut<'a>(&'a mut self) -> &'a mut LtpcRaw<P, C> {
        unsafe {
            // let raw: Pin<&mut LtpcRaw<P, C>> = Pin::as_mut(&mut self.raw);
            // let raw: &mut LtpcRaw<P, C> = Pin::get_unchecked_mut(raw);
            let raw: Pin<&mut LtpcRaw<P, C>> = Pin::as_mut(&mut self.raw);
            let raw: &mut LtpcRaw<P, C> = Pin::get_unchecked_mut(raw);
            raw
        }
    }

    pub fn take_child<'a>(&'a mut self) -> Option<C> {
        // if child is not pin, might be able to take out without unsafe code.
        let raw = self._get_raw_mut();
        raw.child.take()
    }

    pub fn take_parent(mut self) -> P {
        drop(self.take_child());
        let raw = self._get_raw_mut();
        raw.parent.take().unwrap()
    }

    pub fn set_child<'self_, 'params, 'child, F>(&'self_ mut self, f: F)
    where
        C: 'child,
        P: 'params,
        F: FnOnce(&'params mut P) -> C,
    {
        let raw: &'self_ mut LtpcRaw<P, C> = self._get_raw_mut();

        let parent: &'self_ mut P = raw.parent.as_mut().unwrap();

        let parent: &'params mut P = unsafe { std::mem::transmute(parent) };

        let child: C = f(parent);

        raw.child = Some(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(Debug)]
        struct Senior {
            val: String,
        }

        impl Senior {
            pub fn get_junior<'a>(&'a self) -> Junior<'a> {
                Junior { val: &self.val }
            }
        }

        #[derive(Debug)]
        struct Junior<'a> {
            val: &'a str,
        }

        let mut ltpc: Ltpc<Senior, Junior> = Ltpc::new(Senior {
            val: String::from("foo"),
        });

        ltpc.set_child(|senior| senior.get_junior());

        assert_eq!(
            ltpc.parent().val,
            ltpc.child().map(|child| child.val).unwrap_or_default(),
        );
    }
}
