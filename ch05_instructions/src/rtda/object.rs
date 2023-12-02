//! The object reference implementation

#[derive(Clone, Debug)]
pub struct Object;

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self.eq(&_other)
    }
}
