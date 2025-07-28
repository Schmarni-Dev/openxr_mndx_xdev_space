use self::macros::*;
use openxr::{
    Posef, StructureType,
    sys::{self, BaseOutStructure, Bool32, Session, pfn::VoidFunction},
};
use std::{
    ffi::{CStr, c_char},
    mem::{self, MaybeUninit},
    os::raw::c_void,
};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XDevListMNDX(u64);
handle!(XDevListMNDX);
wrapper! {
    #[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    XDevIdMNDX(u64)
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SystemXDevSpacePropertiesMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_xdev_space: Bool32,
}

impl SystemXDevSpacePropertiesMNDX {
    // StructureType::from_raw is not const for some reason...
    pub const TYPE: StructureType = unsafe { mem::transmute(1000444001) };
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CreateXDevListInfoMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
}

impl CreateXDevListInfoMNDX {
    // StructureType::from_raw is not const for some reason...
    pub const TYPE: StructureType = unsafe { mem::transmute(1000444002) };
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GetXDevInfoMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub dev_id: XDevIdMNDX,
}

impl GetXDevInfoMNDX {
    // StructureType::from_raw is not const for some reason...
    pub const TYPE: StructureType = unsafe { mem::transmute(1000444003) };
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XDevPropertiesMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub name: [c_char; 256],
    pub serial: [c_char; 256],
    pub can_create_space: Bool32,
}

impl XDevPropertiesMNDX {
    // StructureType::from_raw is not const for some reason...
    pub const TYPE: StructureType = unsafe { mem::transmute(1000444004) };
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CreateXDevSpaceInfoMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub xdev_list: XDevListMNDX,
    pub xdev_id: XDevIdMNDX,
    pub offset: Posef,
}

impl CreateXDevSpaceInfoMNDX {
    // StructureType::from_raw is not const for some reason...
    pub const TYPE: StructureType = unsafe { mem::transmute(1000444005) };
}
#[rustfmt::skip]
pub type CreateXDevListMNDX = unsafe extern "system" fn(
    session: Session,
    name: *const CreateXDevListInfoMNDX,
    xdev_list: *mut XDevListMNDX,
) -> sys::Result;
#[rustfmt::skip]
pub type GetXDevListGenerationNumberMNDX = unsafe extern "system" fn(
    xdev_list: XDevListMNDX,
    generation: *mut u64
) -> sys::Result;
#[rustfmt::skip]
pub type EnumerateXDevsMNDX = unsafe extern "system" fn(
    xdev_list: XDevListMNDX,
    xdev_capacity_input: u32,
    xdev_capacity_output: *mut u32,
    xdev_ids: *mut XDevIdMNDX,
) -> sys::Result;
#[rustfmt::skip]
pub type GetXDevPropertiesMNDX = unsafe extern "system" fn(
    xdev_list: XDevListMNDX,
    get_xdev_info: *const GetXDevInfoMNDX,
    properties: *mut XDevPropertiesMNDX,
) -> sys::Result;
#[rustfmt::skip]
pub type DestroyXDevListMNDX = unsafe extern "system" fn(
    xdev_list: XDevListMNDX,
) -> sys::Result;
#[rustfmt::skip]
pub type CreateXDevSpaceMNDX = unsafe extern "system" fn(
    session: Session,
    create_xdev_space_info: *const CreateXDevSpaceInfoMNDX,
    space: *mut sys::Space,
) -> sys::Result;

#[derive(Clone, Copy, Debug)]
pub struct XDevSpacesMNDXFunctions {
    pub create_xdev_list: CreateXDevListMNDX,
    pub get_xdev_list_generation_number: GetXDevListGenerationNumberMNDX,
    pub enumerate_xdevs: EnumerateXDevsMNDX,
    pub get_xdev_properties: GetXDevPropertiesMNDX,
    pub destroy_xdev_list: DestroyXDevListMNDX,
    pub create_xdev_space: CreateXDevSpaceMNDX,
}

impl XDevSpacesMNDXFunctions {
    pub fn load(instance: &openxr::Instance) -> openxr::Result<Self> {
        unsafe {
            #[expect(clippy::missing_transmute_annotations)]
            Ok(Self {
                create_xdev_list: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrCreateXDevListMNDX",
                )?),
                get_xdev_list_generation_number: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrGetXDevListGenerationNumberMNDX",
                )?),
                enumerate_xdevs: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrEnumerateXDevsMNDX",
                )?),
                get_xdev_properties: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrGetXDevPropertiesMNDX",
                )?),
                destroy_xdev_list: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrDestroyXDevListMNDX",
                )?),
                create_xdev_space: mem::transmute(get_instance_proc_addr(
                    instance,
                    c"xrCreateXDevSpaceMNDX",
                )?),
            })
        }
    }
}
fn get_instance_proc_addr(
    instance: &openxr::Instance,
    name: &CStr,
) -> openxr::Result<VoidFunction> {
    let mut func = None;
    unsafe {
        cvt((instance.fp().get_instance_proc_addr)(
            instance.as_raw(),
            name.as_ptr(),
            &mut func,
        ))?;
    };
    Ok(match func {
        Some(f) => f,
        None => panic!(
            "unable to load function pointer for {}",
            name.to_string_lossy()
        ),
    })
}
pub(crate) fn cvt(x: sys::Result) -> openxr::Result<sys::Result> {
    if x.into_raw() >= 0 { Ok(x) } else { Err(x) }
}

mod macros {
    macro_rules! handle {
        ($name:ident) => {
            impl $name {
                pub const NULL: Self = Self(0);
                #[inline]
                pub fn from_raw(x: u64) -> Self {
                    Self(x)
                }
                #[inline]
                pub fn into_raw(self) -> u64 {
                    self.0
                }
            }
            impl Default for $name {
                fn default() -> Self {
                    Self::NULL
                }
            }
        };
    }
    macro_rules! wrapper {
        {$(#[$meta: meta])* $ident:ident($ty:ty)} => {
            $(#[$meta])* #[repr(transparent)]
            pub struct $ident($ty);
            impl $ident {
                pub fn from_raw(x: $ty) -> Self { Self(x) }
                pub fn into_raw(self) -> $ty { self.0 }
            }
        }
    }
    pub(super) use handle;
    pub(super) use wrapper;
}
