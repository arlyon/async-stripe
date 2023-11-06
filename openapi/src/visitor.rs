use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::RustType;
use crate::stripe_object::RequestSpec;

pub trait Visitor {
    fn visit_obj(&mut self, obj: &RustObject, _meta: Option<&ObjectMetadata>)
    where
        Self: Sized,
    {
        obj.visit(self);
    }

    fn visit_typ(&mut self, typ: &RustType)
    where
        Self: Sized,
    {
        typ.visit(self);
    }

    fn visit_req(&mut self, req: &RequestSpec)
    where
        Self: Sized,
    {
        req.visit(self);
    }
}

pub trait VisitorMut {
    fn visit_obj_mut(&mut self, obj: &mut RustObject, _meta: Option<&ObjectMetadata>)
    where
        Self: Sized,
    {
        obj.visit_mut(self);
    }

    fn visit_typ_mut(&mut self, typ: &mut RustType)
    where
        Self: Sized,
    {
        typ.visit_mut(self);
    }

    fn visit_req_mut(&mut self, req: &mut RequestSpec)
    where
        Self: Sized,
    {
        req.visit_mut(self);
    }
}
