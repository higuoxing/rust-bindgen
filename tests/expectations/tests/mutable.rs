/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub m_member: ::std::os::raw::c_int,
    pub m_other: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 8usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . m_member as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( m_member ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . m_other as * const _ as usize } ,
                4usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( m_other ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug)]
pub struct NonCopiable {
    pub m_member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NonCopiable() {
    assert_eq!(::std::mem::size_of::<NonCopiable>() , 4usize , concat ! (
               "Size of: " , stringify ! ( NonCopiable ) ));
    assert_eq! (::std::mem::align_of::<NonCopiable>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( NonCopiable ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NonCopiable ) ) . m_member as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NonCopiable ) , "::" ,
                stringify ! ( m_member ) ));
}
#[repr(C)]
#[derive(Debug)]
pub struct NonCopiableWithNonCopiableMutableMember {
    pub m_member: NonCopiable,
}
#[test]
fn bindgen_test_layout_NonCopiableWithNonCopiableMutableMember() {
    assert_eq!(::std::mem::size_of::<NonCopiableWithNonCopiableMutableMember>()
               , 4usize , concat ! (
               "Size of: " , stringify ! (
               NonCopiableWithNonCopiableMutableMember ) ));
    assert_eq! (::std::mem::align_of::<NonCopiableWithNonCopiableMutableMember>()
                , 4usize , concat ! (
                "Alignment of " , stringify ! (
                NonCopiableWithNonCopiableMutableMember ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NonCopiableWithNonCopiableMutableMember )
                ) . m_member as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                NonCopiableWithNonCopiableMutableMember ) , "::" , stringify !
                ( m_member ) ));
}