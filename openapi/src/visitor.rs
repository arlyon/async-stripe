use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::RustType;
use crate::stripe_object::{RequestSpec, StripeObject};

pub trait Visit<'a> {
    fn visit_obj(&mut self, obj: &'a RustObject, _meta: Option<&'a ObjectMetadata>)
    where
        Self: Sized,
    {
        obj.visit(self);
    }

    fn visit_typ(&mut self, typ: &'a RustType)
    where
        Self: Sized,
    {
        typ.visit(self);
    }

    fn visit_req(&mut self, req: &'a RequestSpec)
    where
        Self: Sized,
    {
        req.visit(self);
    }

    fn visit_stripe_object(&mut self, obj: &'a StripeObject)
    where
        Self: Sized,
    {
        obj.visit(self)
    }
}

pub trait VisitMut {
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

    fn visit_stripe_object_mut(&mut self, obj: &mut StripeObject)
    where
        Self: Sized,
    {
        obj.visit_mut(self)
    }
}
