use std::{
    ffi::CStr,
    mem::{self, MaybeUninit},
    ops::Deref,
    ptr,
    sync::Arc,
};

use bindings::{
    CreateXDevListInfoMNDX, CreateXDevSpaceInfoMNDX, GetXDevInfoMNDX,
    SystemXDevSpacePropertiesMNDX, XDevIdMNDX, XDevListMNDX, XDevPropertiesMNDX,
    XDevSpacesMNDXFunctions, cvt,
};
use openxr::{
    AnyGraphics, Posef, Session, SystemId,
    sys::{self, FALSE},
};

pub mod bindings;

pub trait SessionXDevExtensionMNDX {
    fn get_xdev_list(&self) -> openxr::Result<XDevList>;
}
impl<G> SessionXDevExtensionMNDX for openxr::Session<G> {
    fn get_xdev_list(&self) -> openxr::Result<XDevList> {
        let instance = self.instance();
        // loading this every call probably isn't great for perf, oh well
        let funcs = XDevSpacesMNDXFunctions::load(instance)?;
        let mut list = MaybeUninit::<XDevListMNDX>::uninit();
        let info = CreateXDevListInfoMNDX {
            ty: CreateXDevListInfoMNDX::TYPE,
            next: ptr::null_mut(),
        };
        unsafe {
            cvt((funcs.create_xdev_list)(
                self.as_raw(),
                &info,
                list.as_mut_ptr(),
            ))?;
        }
        let list = unsafe { list.assume_init() };

        Ok(XDevList(Arc::new(XDevListInner {
            xdev_funcs: funcs,
            handle: list,
            session: self.clone().into_any_graphics(),
        })))
    }
}
pub trait InstanceXDevExtensionMNDX {
    fn supports_mndx_xdev_spaces(&self, system: SystemId) -> openxr::Result<bool>;
}

impl InstanceXDevExtensionMNDX for openxr::Instance {
    fn supports_mndx_xdev_spaces(&self, system: SystemId) -> openxr::Result<bool> {
        unsafe {
            let mut xdev_props = SystemXDevSpacePropertiesMNDX {
                ty: SystemXDevSpacePropertiesMNDX::TYPE,
                next: ptr::null_mut(),
                supports_xdev_space: FALSE,
            };
            let mut p = sys::SystemProperties {
                ty: sys::SystemProperties::TYPE,
                next: &mut xdev_props as *mut _ as *mut _,
                ..mem::zeroed()
            };
            cvt((self.fp().get_system_properties)(
                self.as_raw(),
                system,
                &mut p,
            ))?;
            Ok(xdev_props.supports_xdev_space.into())
        }
    }
}

#[derive(Clone)]
pub struct XDevList(Arc<XDevListInner>);
impl XDevList {
    pub fn enumerate_xdevs(&self) -> openxr::Result<Vec<XDev>> {
        let mut capacity = 0u32;
        unsafe {
            cvt((self.xdev_funcs.enumerate_xdevs)(
                self.handle,
                0,
                &mut capacity,
                ptr::null_mut(),
            ))?;
        };
        let mut out = vec![XDevIdMNDX::from_raw(0); capacity as usize];

        let mut out_len = 0u32;
        unsafe {
            cvt((self.xdev_funcs.enumerate_xdevs)(
                self.handle,
                capacity,
                &mut out_len,
                out.as_mut_ptr(),
            ))?;
        };
        out.truncate(out_len as usize);
        let mut xdevs = Vec::with_capacity(out_len as usize);
        for id in out {
            let mut out = XDevPropertiesMNDX::out(ptr::null_mut());
            let info = GetXDevInfoMNDX {
                ty: GetXDevInfoMNDX::TYPE,
                next: ptr::null_mut(),
                dev_id: id,
            };
            unsafe {
                cvt((self.xdev_funcs.get_xdev_properties)(
                    self.handle,
                    &info,
                    out.as_mut_ptr(),
                ))?
            };
            let props = unsafe { out.assume_init() };
            xdevs.push(XDev {
                list: self.0.clone(),
                id,
                name: unsafe { CStr::from_ptr(props.name.as_ptr()) }
                    .to_str()
                    .unwrap()
                    .to_string(),
                serial: unsafe { CStr::from_ptr(props.serial.as_ptr()) }
                    .to_str()
                    .unwrap()
                    .to_string(),
                can_create_space: props.can_create_space.into(),
            });
        }
        Ok(xdevs)
    }
    pub fn get_generation(&self) -> openxr::Result<u64> {
        let mut generation = 0u64;
        unsafe {
            cvt((self.xdev_funcs.get_xdev_list_generation_number)(
                self.handle,
                &mut generation,
            ))?
        };
        Ok(generation)
    }
    pub fn session(&self) -> &Session<AnyGraphics> {
        &self.session
    }
}
pub struct XDev {
    list: Arc<XDevListInner>,
    id: XDevIdMNDX,
    name: String,
    serial: String,
    can_create_space: bool,
}
impl XDev {
    /// # Panic:
    /// panics if this xdev cannot be used to create a space, check XDev::can_create_xdev first
    pub fn create_space(&self, offset: Posef) -> openxr::Result<openxr::Space> {
        if !self.can_create_space {
            panic!("this xdev cannot be used to create a Space")
        }
        let info = CreateXDevSpaceInfoMNDX {
            ty: CreateXDevSpaceInfoMNDX::TYPE,
            next: ptr::null_mut(),
            xdev_list: self.list.handle,
            xdev_id: self.id,
            offset,
        };
        let mut space = sys::Space::NULL;
        unsafe {
            cvt((self.list.xdev_funcs.create_xdev_space)(
                self.list.session.as_raw(),
                &info,
                &mut space,
            ))?
        };
        Ok(unsafe { openxr::Space::reference_from_raw(self.list.session.clone(), space) })
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn serial(&self) -> &str {
        &self.serial
    }
    pub fn can_create_space(&self) -> bool {
        self.can_create_space
    }
}

pub struct XDevListInner {
    handle: XDevListMNDX,
    xdev_funcs: XDevSpacesMNDXFunctions,
    session: Session<AnyGraphics>,
}

impl Deref for XDevList {
    type Target = XDevListInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for XDevListInner {
    fn drop(&mut self) {
        unsafe { (self.xdev_funcs.destroy_xdev_list)(self.handle) };
    }
}
