/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct student_ {
    pub name: *mut ::std::os::raw::c_char,
    pub age: ::std::os::raw::c_int,
    pub year: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_student_() {
    assert_eq!(
        ::std::mem::size_of::<student_>(),
        16usize,
        concat!("Size of: ", stringify!(student_))
    );
    assert_eq!(
        ::std::mem::align_of::<student_>(),
        8usize,
        concat!("Alignment of ", stringify!(student_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<student_>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(student_),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<student_>())).age as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(student_),
            "::",
            stringify!(age)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<student_>())).year as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(student_),
            "::",
            stringify!(year)
        )
    );
}
pub type PhdStudent = student_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct professor_ {
    pub name: *mut ::std::os::raw::c_char,
    pub favorite_food: *mut ::std::os::raw::c_char,
    pub likes_nutella: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_professor_() {
    assert_eq!(
        ::std::mem::size_of::<professor_>(),
        24usize,
        concat!("Size of: ", stringify!(professor_))
    );
    assert_eq!(
        ::std::mem::align_of::<professor_>(),
        8usize,
        concat!("Alignment of ", stringify!(professor_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<professor_>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(professor_),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<professor_>())).favorite_food as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(professor_),
            "::",
            stringify!(favorite_food)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<professor_>())).likes_nutella as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(professor_),
            "::",
            stringify!(likes_nutella)
        )
    );
}
pub type Professor = professor_;
extern "C" {
    #[link_name = "\u{1}_NewCody"]
    pub fn NewCody() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewEgan"]
    pub fn NewEgan() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewKaisheng"]
    pub fn NewKaisheng() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewKexin"]
    pub fn NewKexin() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewKraft"]
    pub fn NewKraft() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewPT"]
    pub fn NewPT() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewSahaana"]
    pub fn NewSahaana() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewShoumik"]
    pub fn NewShoumik() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewDDKang"]
    pub fn NewDDKang() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewDeepak"]
    pub fn NewDeepak() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewDeepti"]
    pub fn NewDeepti() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewFir"]
    pub fn NewFir() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewJimbo"]
    pub fn NewJimbo() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewKeshu"]
    pub fn NewKeshu() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewAnkit"]
    pub fn NewAnkit() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewSaachi"]
    pub fn NewSaachi() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewJustin"]
    pub fn NewJustin() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewSwetha"]
    pub fn NewSwetha() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewAnimesh"]
    pub fn NewAnimesh() -> PhdStudent;
}
extern "C" {
    #[link_name = "\u{1}_NewMatei"]
    pub fn NewMatei() -> Professor;
}
extern "C" {
    #[link_name = "\u{1}_NewPeter"]
    pub fn NewPeter() -> Professor;
}