extern {
    fn __sincos_stret(arg1 : f64) -> __double2;
    fn __sincosf_stret(arg1 : f32) -> __float2;
    fn __sincospi_stret(arg1 : f64) -> __double2;
    fn __sincospif_stret(arg1 : f32) -> __float2;
    static mut __stdoutp : *mut __sFILE;
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn acos(arg1 : f64) -> f64;
    fn cos(arg1 : f64) -> f64;
    fn fabs(arg1 : f64) -> f64;
    fn fprintf(arg1 : *mut __sFILE, arg2 : *const u8, ...) -> i32;
    fn sin(arg1 : f64) -> f64;
    fn sprintf(arg1 : *mut u8, arg2 : *const u8, ...) -> i32;
    fn sqrt(arg1 : f64) -> f64;
    fn tan(arg1 : f64) -> f64;
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        ({
             let _rhs = _c;
             let _lhs
                 = &mut *{
                             let _old = (*_p)._p;
                             (*_p)._p = (*_p)._p.offset(1isize);
                             _old
                         };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct __float2 {
    pub __sinval : f32,
    pub __cosval : f32,
}

impl Clone for __float2 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sincosf(
    mut __x : f32, mut __sinp : *mut f32, mut __cosp : *mut f32
) {
    let __stret : __float2 = __sincosf_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}

#[derive(Copy)]
#[repr(C)]
pub struct __double2 {
    pub __sinval : f64,
    pub __cosval : f64,
}

impl Clone for __double2 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sincos(
    mut __x : f64, mut __sinp : *mut f64, mut __cosp : *mut f64
) {
    let __stret : __double2 = __sincos_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}

#[no_mangle]
pub unsafe extern fn __sincospif(
    mut __x : f32, mut __sinp : *mut f32, mut __cosp : *mut f32
) {
    let __stret : __float2 = __sincospif_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}

#[no_mangle]
pub unsafe extern fn __sincospi(
    mut __x : f64, mut __sinp : *mut f64, mut __cosp : *mut f64
) {
    let __stret : __double2 = __sincospi_stret(__x);
    *__sinp = __stret.__sinval;
    *__cosp = __stret.__cosval;
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 { 0i32 }

#[no_mangle]
pub unsafe extern fn mr_Print(mut str : *const u8) {
    fprintf(__stdoutp,(*b"%s\0").as_ptr(),str);
}

#[no_mangle]
pub unsafe extern fn mr_PrintMatrix(
    mut m : *mut f64, n1 : i32, n2 : i32
) {
    let mut i : i32;
    let mut j : i32;
    let mut str : [u8; 16];
    sprintf(str.as_mut_ptr(),(*b"\n\0").as_ptr());
    mr_Print(str.as_mut_ptr() as (*const u8));
    i = 0i32;
    'loop1: loop {
        if !(i < n1) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n2) {
                break;
            }
            sprintf(
                str.as_mut_ptr(),
                (*b"%8.4f \0").as_ptr(),
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 }
            );
            mr_Print(str.as_mut_ptr() as (*const u8));
            j = j + 1;
        }
        sprintf(str.as_mut_ptr(),(*b"\n\0").as_ptr());
        mr_Print(str.as_mut_ptr() as (*const u8));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_PrintVector(mut v : *const f64, n : i32) {
    let mut i : i32;
    let mut str : [u8; 16];
    sprintf(str.as_mut_ptr(),(*b"\n\0").as_ptr());
    mr_Print(str.as_mut_ptr() as (*const u8));
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        sprintf(
            str.as_mut_ptr(),
            (*b"%10.4f \0").as_ptr(),
            *{
                 let _old = v;
                 v = v.offset(1isize);
                 _old
             }
        );
        mr_Print(str.as_mut_ptr() as (*const u8));
        i = i + 1;
    }
    sprintf(str.as_mut_ptr(),(*b"\n\0").as_ptr());
    mr_Print(str.as_mut_ptr() as (*const u8));
}

#[no_mangle]
pub unsafe extern fn mr_PrintDouble(x : f64) {
    let mut str : [u8; 16];
    sprintf(str.as_mut_ptr(),(*b"\n%10.4f\n\0").as_ptr(),x);
    mr_Print(str.as_mut_ptr() as (*const u8));
}

#[no_mangle]
pub unsafe extern fn mr_ZeroMatrix(
    mut m : *mut f64, mut n1 : i32, mut n2 : i32
) {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n1 * n2) {
            break;
        }
        *{
             let _old = m;
             m = m.offset(1isize);
             _old
         } = 0i32 as (f64);
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_Id(mut m : *mut f64, mut n : i32) {
    let mut i : i32;
    let mut j : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n) {
                break;
            }
            if i == j {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = 1i32 as (f64);
            } else {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = 0i32 as (f64);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_IdxS(
    mut m : *mut f64, mut n : i32, mut s : f64
) {
    let mut i : i32;
    let mut j : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n) {
                break;
            }
            if i == j {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = s;
            } else {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = 0i32 as (f64);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_Diag(
    mut m : *mut f64, mut n : i32, mut s : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n) {
                break;
            }
            if i == j {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = *{
                          let _old = s;
                          s = s.offset(1isize);
                          _old
                      };
            } else {
                *{
                     let _old = m;
                     m = m.offset(1isize);
                     _old
                 } = 0i32 as (f64);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_CopyMatrix(
    mut s : *const f64, n1 : i32, n2 : i32, mut d : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n1) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n2) {
                break;
            }
            *{
                 let _old = d;
                 d = d.offset(1isize);
                 _old
             } = *{
                      let _old = s;
                      s = s.offset(1isize);
                      _old
                  };
            j = j + 1;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_CopyVector(
    mut s : *const f64, n : i32, mut d : *mut f64
) {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = s;
                  s = s.offset(1isize);
                  _old
              };
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_ExtractMatrix(
    mut s : *mut f64,
    n1 : i32,
    n2 : i32,
    r0 : i32,
    c0 : i32,
    nr : i32,
    nc : i32,
    mut d : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    let mut st : *mut f64;
    st = s.offset(c0 as (isize)).offset((n2 * r0) as (isize));
    i = 0i32;
    'loop1: loop {
        if !(i < nr) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < nc) {
                break;
            }
            *{
                 let _old = d;
                 d = d.offset(1isize);
                 _old
             } = *{
                      let _old = st;
                      st = st.offset(1isize);
                      _old
                  };
            j = j + 1;
        }
        st = st.offset((n2 - nc) as (isize));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_InsertMatrix(
    mut s : *const f64,
    n1s : i32,
    n2s : i32,
    mut d : *mut f64,
    n1d : i32,
    n2d : i32,
    r0 : i32,
    c0 : i32
) {
    let mut i : i32;
    let mut j : i32;
    let mut dt : *mut f64;
    dt = d.offset(c0 as (isize)).offset((r0 * n2d) as (isize));
    i = 0i32;
    'loop1: loop {
        if !(i < n1s) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n2s) {
                break;
            }
            *{
                 let _old = dt;
                 dt = dt.offset(1isize);
                 _old
             } = *{
                      let _old = s;
                      s = s.offset(1isize);
                      _old
                  };
            j = j + 1;
        }
        dt = dt.offset((n2d - n2s) as (isize));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_Transpose(
    mut s : *const f64, n1 : i32, n2 : i32, mut d : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    let mut nextcol : i32;
    nextcol = 1i32 - n1 * n2;
    i = 0i32;
    'loop1: loop {
        if !(i < n1) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n2) {
                break;
            }
            *d = *{
                      let _old = s;
                      s = s.offset(1isize);
                      _old
                  };
            d = d.offset(n1 as (isize));
            j = j + 1;
        }
        d = d.offset(nextcol as (isize));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_AddVectors(
    mut v1 : *const f64,
    mut v2 : *const f64,
    n : i32,
    mut v3 : *mut f64
) {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        *{
             let _old = v3;
             v3 = v3.offset(1isize);
             _old
         } = *{
                  let _old = v1;
                  v1 = v1.offset(1isize);
                  _old
              } + *{
                       let _old = v2;
                       v2 = v2.offset(1isize);
                       _old
                   };
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_SubtractVectors(
    mut v1 : *const f64,
    mut v2 : *const f64,
    n : i32,
    mut v3 : *mut f64
) {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        *{
             let _old = v3;
             v3 = v3.offset(1isize);
             _old
         } = *{
                  let _old = v1;
                  v1 = v1.offset(1isize);
                  _old
              } - *{
                       let _old = v2;
                       v2 = v2.offset(1isize);
                       _old
                   };
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_AddMatrices(
    mut s1 : *const f64,
    mut s2 : *const f64,
    n1 : i32,
    n2 : i32,
    mut d : *mut f64
) {
    let mut i : i32;
    let mut len : i32;
    len = n1 * n2;
    i = 0i32;
    'loop1: loop {
        if !(i < len) {
            break;
        }
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = s1;
                  s1 = s1.offset(1isize);
                  _old
              } + *{
                       let _old = s2;
                       s2 = s2.offset(1isize);
                       _old
                   };
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_MS(
    mut m : *const f64, n1 : i32, n2 : i32, s : f64, mut d : *mut f64
) {
    let mut i : i32;
    let mut len : i32;
    len = n1 * n2;
    i = 0i32;
    'loop1: loop {
        if !(i < len) {
            break;
        }
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = m;
                  m = m.offset(1isize);
                  _old
              } * s;
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_so3xS(
    wb : *mut f64, s : f64, mut d : *mut f64
) {
    let mut wx : f64;
    let mut wy : f64;
    let mut wz : f64;
    wx = *wb.offset(7isize) * s;
    wy = *wb.offset(2isize) * s;
    wz = *wb.offset(3isize) * s;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wy;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -wx;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -wy;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx;
    *d = 0i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_VS(
    mut v : *const f64, n : i32, s : f64, mut d : *mut f64
) {
    let mut i : i32;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = v;
                  v = v.offset(1isize);
                  _old
              } * s;
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_MV(
    mut m : *const f64,
    n1 : i32,
    n2 : i32,
    mut v : *const f64,
    mut d : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    let mut sum : f64;
    i = 0i32;
    'loop1: loop {
        if !(i < n1) {
            break;
        }
        sum = 0i32 as (f64);
        j = 0i32;
        'loop4: loop {
            if !(j < n2) {
                break;
            }
            sum = sum + *{
                             let _old = m;
                             m = m.offset(1isize);
                             _old
                         } * *{
                                  let _old = v;
                                  v = v.offset(1isize);
                                  _old
                              };
            j = j + 1;
        }
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = sum;
        v = v.offset(-(n2 as (isize)));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_SE3xSE3(
    Ta : *mut f64, Tb : *mut f64, mut Tc : *mut f64
) {
    let mut ta11 : f64;
    let mut ta12 : f64;
    let mut ta13 : f64;
    let mut ta14 : f64;
    let mut ta21 : f64;
    let mut ta22 : f64;
    let mut ta23 : f64;
    let mut ta24 : f64;
    let mut ta31 : f64;
    let mut ta32 : f64;
    let mut ta33 : f64;
    let mut ta34 : f64;
    let mut tb11 : f64;
    let mut tb12 : f64;
    let mut tb13 : f64;
    let mut tb14 : f64;
    let mut tb21 : f64;
    let mut tb22 : f64;
    let mut tb23 : f64;
    let mut tb24 : f64;
    let mut tb31 : f64;
    let mut tb32 : f64;
    let mut tb33 : f64;
    let mut tb34 : f64;
    ta11 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta12 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta13 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta14 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta21 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta22 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta23 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta24 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta31 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta32 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta33 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    ta34 = *{
                let _old = Ta;
                Ta = Ta.offset(1isize);
                _old
            };
    tb11 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb12 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb13 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb14 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb21 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb22 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb23 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb24 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb31 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb32 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb33 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    tb34 = *{
                let _old = Tb;
                Tb = Tb.offset(1isize);
                _old
            };
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta11 * tb11 + ta12 * tb21 + ta13 * tb31;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta11 * tb12 + ta12 * tb22 + ta13 * tb32;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta11 * tb13 + ta12 * tb23 + ta13 * tb33;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta11 * tb14 + ta12 * tb24 + ta13 * tb34 + ta14;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta21 * tb11 + ta22 * tb21 + ta23 * tb31;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta21 * tb12 + ta22 * tb22 + ta23 * tb32;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta21 * tb13 + ta22 * tb23 + ta23 * tb33;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta21 * tb14 + ta22 * tb24 + ta23 * tb34 + ta24;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta31 * tb11 + ta32 * tb21 + ta33 * tb31;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta31 * tb12 + ta32 * tb22 + ta33 * tb32;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta31 * tb13 + ta32 * tb23 + ta33 * tb33;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = ta31 * tb14 + ta32 * tb24 + ta33 * tb34 + ta34;
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = Tc;
         Tc = Tc.offset(1isize);
         _old
     } = 0i32 as (f64);
    *Tc = 1i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_so3Squared(s : *mut f64, mut d : *mut f64) {
    let mut wx : f64;
    let mut wy : f64;
    let mut wz : f64;
    let mut wx2m : f64;
    let mut wy2m : f64;
    let mut wz2m : f64;
    let mut wxwy : f64;
    let mut wxwz : f64;
    let mut wywz : f64;
    wx = *s.offset(7isize);
    wy = *s.offset(2isize);
    wz = *s.offset(3isize);
    wx2m = -wx * wx;
    wy2m = -wy * wy;
    wz2m = -wz * wz;
    wxwy = wx * wy;
    wxwz = wx * wz;
    wywz = wy * wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wy2m + wz2m;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wxwy;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wxwz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wxwy;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx2m + wz2m;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wywz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wxwz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wywz;
    *d = wx2m + wy2m;
}

#[no_mangle]
pub unsafe extern fn mr_so3xso3(
    w1 : *mut f64, w2 : *mut f64, mut d : *mut f64
) {
    let mut wx1 : f64;
    let mut wy1 : f64;
    let mut wz1 : f64;
    let mut wx2 : f64;
    let mut wy2 : f64;
    let mut wz2 : f64;
    wx1 = *w1.offset(7isize);
    wy1 = *w1.offset(2isize);
    wz1 = *w1.offset(3isize);
    wx2 = *w1.offset(7isize);
    wy2 = *w1.offset(2isize);
    wz2 = *w1.offset(3isize);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -wy1 * wy2 - wz1 * wz2;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx2 * wy1;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx2 * wz1;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx1 * wy2;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -wx1 * wx2 - wx1 * wz2;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx2 * wz1;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wx1 * wz2;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = wy1 * wz2;
    *d = -wx1 * wx2 - wy1 * wy2;
}

#[no_mangle]
pub unsafe extern fn mr_so3xSO3(
    w : *mut f64, R : *mut f64, mut d : *mut f64
) {
    let mut wx : f64;
    let mut wy : f64;
    let mut wz : f64;
    let mut r11 : f64;
    let mut r12 : f64;
    let mut r13 : f64;
    let mut r21 : f64;
    let mut r22 : f64;
    let mut r23 : f64;
    let mut r31 : f64;
    let mut r32 : f64;
    let mut r33 : f64;
    wx = *w.offset(7isize);
    wy = *w.offset(2isize);
    wz = *w.offset(3isize);
    r11 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r12 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r13 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r21 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r22 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r23 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r31 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r32 = *{
               let _old = R;
               R = R.offset(1isize);
               _old
           };
    r33 = *R;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r31 * wy - r21 * wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r32 * wy - r22 * wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r33 * wy - r23 * wz;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r11 * wz - r31 * wx;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r12 * wz - r32 * wx;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r13 * wz - r33 * wx;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r21 * wz - r11 * wy;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = r22 * wz - r12 * wy;
    *d = r23 * wz - r13 * wy;
}

#[no_mangle]
pub unsafe extern fn mr_MM(
    mut m1 : *const f64,
    mut m2 : *mut f64,
    n1 : i32,
    n2 : i32,
    n3 : i32,
    mut d : *mut f64
) {
    let mut i : i32;
    let mut j : i32;
    let mut k : i32;
    let mut nextm2col : i32 = 1i32 - n2 * n3;
    let mut sum : f64;
    let mut m2t : *mut f64;
    m2t = m2;
    i = 0i32;
    'loop1: loop {
        if !(i < n1) {
            break;
        }
        j = 0i32;
        'loop4: loop {
            if !(j < n3) {
                break;
            }
            sum = 0i32 as (f64);
            k = 0i32;
            'loop8: loop {
                if !(k < n2) {
                    break;
                }
                sum = sum + *{
                                 let _old = m1;
                                 m1 = m1.offset(1isize);
                                 _old
                             } * *m2t;
                m2t = m2t.offset(n3 as (isize));
                k = k + 1;
            }
            *{
                 let _old = d;
                 d = d.offset(1isize);
                 _old
             } = sum;
            m2t = m2t.offset(nextm2col as (isize));
            m1 = m1.offset(-(n2 as (isize)));
            j = j + 1;
        }
        m2t = m2;
        m1 = m1.offset(n2 as (isize));
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_Norm(mut v : *const f64, mut n : i32) -> f64 {
    let mut i : i32;
    let mut ss : f64 = 0i32 as (f64);
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        ss = ss + *v * *v;
        v = v.offset(1isize);
        i = i + 1;
    }
    sqrt(ss)
}

#[no_mangle]
pub unsafe extern fn mr_NearZero(val : f64) -> i32 {
    (fabs(val) < 1e-6f64) as (i32)
}

#[no_mangle]
pub unsafe extern fn mr_MatrixIsZero(
    mut m : *const f64, n1 : i32, n2 : i32
) -> i32 {
    let mut _currentBlock;
    let mut i : i32;
    let mut n : i32 = n1 * n2;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            _currentBlock = 2;
            break;
        }
        if mr_NearZero(
               *{
                    let _old = m;
                    m = m.offset(1isize);
                    _old
                }
           ) == 0 {
            _currentBlock = 5;
            break;
        }
        i = i + 1;
    }
    if _currentBlock == 2 { 1i32 } else { 0i32 }
}

#[no_mangle]
pub unsafe extern fn mr_Trace(mut m : *const f64, n : i32) -> f64 {
    let mut i : i32;
    let mut tr : f64 = 0.0f64;
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        tr = tr + *m.offset((i + i * n) as (isize));
        i = i + 1;
    }
    tr
}

#[no_mangle]
pub unsafe extern fn mr_getCofactor(
    mut m : *const f64,
    p : i32,
    q : i32,
    n : i32,
    dim : i32,
    mut d : *mut f64
) {
    let mut i : i32 = 0i32;
    let mut j : i32 = 0i32;
    let mut row : i32;
    let mut col : i32;
    row = 0i32;
    'loop1: loop {
        if !(row < n) {
            break;
        }
        col = 0i32;
        'loop4: loop {
            if !(col < n) {
                break;
            }
            if row != p && (col != q) {
                *d.offset((i * dim) as (isize)).offset(j as (isize)) = *m.offset(
                                                                            (row * dim) as (isize)
                                                                        ).offset(
                                                                            col as (isize)
                                                                        );
                j = j + 1;
                if j == n - 1i32 {
                    j = 0i32;
                    i = i + 1;
                }
            }
            col = col + 1;
        }
        row = row + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_RotInv(s : *mut f64, mut d : *mut f64) {
    mr_Transpose(s as (*const f64),3i32,3i32,d);
}

#[no_mangle]
pub unsafe extern fn mr_TransToRp(
    s : *mut f64, mut R : *mut f64, mut p : *mut f64
) {
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = p;
         p = p.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = p;
         p = p.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = R;
         R = R.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *R = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *p = *s;
}

#[no_mangle]
pub unsafe extern fn mr_RpToTrans(
    R : *mut f64, p : *mut f64, mut T : *mut f64
) {
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = p;
              p = p.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = p;
              p = p.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *{
              let _old = R;
              R = R.offset(1isize);
              _old
          };
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *R;
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = *p;
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = T;
         T = T.offset(1isize);
         _old
     } = 0i32 as (f64);
    *T = 1i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_TransInv(s : *mut f64, mut d : *mut f64) {
    let mut Rs : [f64; 9];
    let mut RTs : [f64; 9];
    let mut ps : [f64; 3];
    let mut RTps : [f64; 3];
    let mut R : *mut f64;
    let mut RT : *mut f64;
    let mut p : *mut f64;
    let mut RTp : *mut f64;
    R = Rs.as_mut_ptr();
    RT = RTs.as_mut_ptr();
    p = ps.as_mut_ptr();
    RTp = RTps.as_mut_ptr();
    mr_TransToRp(s,R,p);
    mr_Transpose(R as (*const f64),3i32,3i32,RT);
    mr_MV(RT as (*const f64),3i32,3i32,p as (*const f64),RTp);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -*{
               let _old = RTp;
               RTp = RTp.offset(1isize);
               _old
           };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -*{
               let _old = RTp;
               RTp = RTp.offset(1isize);
               _old
           };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *{
              let _old = RT;
              RT = RT.offset(1isize);
              _old
          };
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = *RT;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = -*RTp;
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = d;
         d = d.offset(1isize);
         _old
     } = 0i32 as (f64);
    *d = 1i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_se3ToVec(se3mat : *mut f64, mut V : *mut f64) {
    *{
         let _old = V;
         V = V.offset(1isize);
         _old
     } = *se3mat.offset(9isize);
    *{
         let _old = V;
         V = V.offset(1isize);
         _old
     } = *se3mat.offset(2isize);
    *{
         let _old = V;
         V = V.offset(1isize);
         _old
     } = *se3mat.offset(4isize);
    *{
         let _old = V;
         V = V.offset(1isize);
         _old
     } = *se3mat.offset(3isize);
    *{
         let _old = V;
         V = V.offset(1isize);
         _old
     } = *se3mat.offset(7isize);
    *V = *se3mat.offset(11isize);
}

#[no_mangle]
pub unsafe extern fn mr_so3ToVec(so3mat : *mut f64, mut w : *mut f64) {
    *{
         let _old = w;
         w = w.offset(1isize);
         _old
     } = *so3mat.offset(7isize);
    *{
         let _old = w;
         w = w.offset(1isize);
         _old
     } = *so3mat.offset(2isize);
    *w = *so3mat.offset(3isize);
}

#[no_mangle]
pub unsafe extern fn mr_VecToso3(w : *mut f64, mut so3mat : *mut f64) {
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = -*w.offset(2isize);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = *w.offset(1isize);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = *w.offset(2isize);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = -*w.offset(0isize);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = -*w.offset(1isize);
    *{
         let _old = so3mat;
         so3mat = so3mat.offset(1isize);
         _old
     } = *w.offset(0isize);
    *so3mat = 0i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_VecTose3(V : *mut f64, mut se3mat : *mut f64) {
    let mut wbracket : [f64; 9];
    mr_VecToso3(V,wbracket.as_mut_ptr());
    mr_InsertMatrix(
        wbracket.as_mut_ptr() as (*const f64),
        3i32,
        3i32,
        se3mat,
        4i32,
        4i32,
        0i32,
        0i32
    );
    *se3mat.offset(3isize) = *V.offset(3isize);
    *se3mat.offset(7isize) = *V.offset(4isize);
    *se3mat.offset(11isize) = *V.offset(5isize);
    *se3mat.offset(12isize) = 0i32 as (f64);
    *se3mat.offset(13isize) = 0i32 as (f64);
    *se3mat.offset(14isize) = 0i32 as (f64);
    *se3mat.offset(15isize) = 0i32 as (f64);
}

#[no_mangle]
pub unsafe extern fn mr_Adjoint(
    T : *mut f64, V1 : *mut f64, mut V2 : *mut f64
) {
    let mut r11 : f64;
    let mut r12 : f64;
    let mut r13 : f64;
    let mut r21 : f64;
    let mut r22 : f64;
    let mut r23 : f64;
    let mut r31 : f64;
    let mut r32 : f64;
    let mut r33 : f64;
    let mut px : f64;
    let mut py : f64;
    let mut pz : f64;
    let mut wx : f64;
    let mut wy : f64;
    let mut wz : f64;
    let mut vx : f64;
    let mut vy : f64;
    let mut vz : f64;
    let mut r11wx : f64;
    let mut r12wy : f64;
    let mut r13wz : f64;
    let mut r21wx : f64;
    let mut r22wy : f64;
    let mut r23wz : f64;
    let mut r31wx : f64;
    let mut r32wy : f64;
    let mut r33wz : f64;
    r11 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r12 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r13 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    px = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r21 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r22 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r23 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    py = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r31 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r32 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r33 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    pz = *T;
    wx = *{
              let _old = V1;
              V1 = V1.offset(1isize);
              _old
          };
    wy = *{
              let _old = V1;
              V1 = V1.offset(1isize);
              _old
          };
    wz = *{
              let _old = V1;
              V1 = V1.offset(1isize);
              _old
          };
    vx = *{
              let _old = V1;
              V1 = V1.offset(1isize);
              _old
          };
    vy = *{
              let _old = V1;
              V1 = V1.offset(1isize);
              _old
          };
    vz = *V1;
    r11wx = r11 * wx;
    r12wy = r12 * wy;
    r13wz = r13 * wz;
    r21wx = r21 * wx;
    r22wy = r22 * wy;
    r23wz = r23 * wz;
    r31wx = r31 * wx;
    r32wy = r32 * wy;
    r33wz = r33 * wz;
    *{
         let _old = V2;
         V2 = V2.offset(1isize);
         _old
     } = r11wx + r12wy + r13wz;
    *{
         let _old = V2;
         V2 = V2.offset(1isize);
         _old
     } = r21wx + r22wy + r23wz;
    *{
         let _old = V2;
         V2 = V2.offset(1isize);
         _old
     } = r31wx + r32wy + r33wz;
    *{
         let _old = V2;
         V2 = V2.offset(1isize);
         _old
     } = r11 * vx + r12 * vy + r13 * vz + py * (r31wx + r32wy + r33wz) - pz * (r21wx + r22wy + r23wz);
    *{
         let _old = V2;
         V2 = V2.offset(1isize);
         _old
     } = r21 * vx + r22 * vy + r23 * vz + pz * (r11wx + r12wy + r13wz) - px * (r31wx + r32wy + r33wz);
    *V2 = r31 * vx + r32 * vy + r33 * vz + px * (r21wx + r22wy + r23wz) - py * (r11wx + r12wy + r13wz);
}

#[no_mangle]
pub unsafe extern fn mr_AdT(T : *mut f64, mut AdT : *mut f64) {
    let mut r11 : f64;
    let mut r12 : f64;
    let mut r13 : f64;
    let mut r21 : f64;
    let mut r22 : f64;
    let mut r23 : f64;
    let mut r31 : f64;
    let mut r32 : f64;
    let mut r33 : f64;
    let mut px : f64;
    let mut py : f64;
    let mut pz : f64;
    r11 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r12 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r13 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    px = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r21 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r22 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r23 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    py = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r31 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r32 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r33 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    pz = *T;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r11;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r12;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r13;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r21;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r22;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r23;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r31;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r32;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r33;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = 0i32 as (f64);
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = py * r31 - pz * r21;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = py * r32 - pz * r22;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = py * r33 - pz * r23;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r11;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r12;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r13;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = pz * r11 - px * r31;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = pz * r12 - px * r32;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = pz * r13 - px * r33;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r21;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r22;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r23;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = px * r21 - py * r11;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = px * r22 - py * r12;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = px * r23 - py * r13;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r31;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r32;
    *{
         let _old = AdT;
         AdT = AdT.offset(1isize);
         _old
     } = r33;
}

#[no_mangle]
pub unsafe extern fn mr_AdjointT(
    T : *mut f64, F1 : *mut f64, mut F2 : *mut f64
) {
    let mut r11 : f64;
    let mut r12 : f64;
    let mut r13 : f64;
    let mut r21 : f64;
    let mut r22 : f64;
    let mut r23 : f64;
    let mut r31 : f64;
    let mut r32 : f64;
    let mut r33 : f64;
    let mut px : f64;
    let mut py : f64;
    let mut pz : f64;
    let mut wx : f64;
    let mut wy : f64;
    let mut wz : f64;
    let mut vx : f64;
    let mut vy : f64;
    let mut vz : f64;
    r11 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r12 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r13 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    px = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r21 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r22 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r23 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    py = *{
              let _old = T;
              T = T.offset(1isize);
              _old
          };
    r31 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r32 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    r33 = *{
               let _old = T;
               T = T.offset(1isize);
               _old
           };
    pz = *T;
    wx = *{
              let _old = F1;
              F1 = F1.offset(1isize);
              _old
          };
    wy = *{
              let _old = F1;
              F1 = F1.offset(1isize);
              _old
          };
    wz = *{
              let _old = F1;
              F1 = F1.offset(1isize);
              _old
          };
    vx = *{
              let _old = F1;
              F1 = F1.offset(1isize);
              _old
          };
    vy = *{
              let _old = F1;
              F1 = F1.offset(1isize);
              _old
          };
    vz = *F1;
    *{
         let _old = F2;
         F2 = F2.offset(1isize);
         _old
     } = vx * (py * r31 - pz * r21) + vy * (pz * r11 - px * r31) + vz * (px * r21 - py * r11) + r11 * wx + r21 * wy + r31 * wz;
    *{
         let _old = F2;
         F2 = F2.offset(1isize);
         _old
     } = vx * (py * r32 - pz * r22) + vy * (pz * r12 - px * r32) + vz * (px * r22 - py * r12) + r12 * wx + r22 * wy + r32 * wz;
    *{
         let _old = F2;
         F2 = F2.offset(1isize);
         _old
     } = vx * (py * r33 - pz * r23) + vy * (pz * r13 - px * r33) + vz * (px * r23 - py * r13) + r13 * wx + r23 * wy + r33 * wz;
    *{
         let _old = F2;
         F2 = F2.offset(1isize);
         _old
     } = r11 * vx + r21 * vy + r31 * vz;
    *{
         let _old = F2;
         F2 = F2.offset(1isize);
         _old
     } = r12 * vx + r22 * vy + r32 * vz;
    *F2 = r13 * vx + r23 * vy + r33 * vz;
}

#[no_mangle]
pub unsafe extern fn mr_AxisAng3(
    mut w : *mut f64, mut th : *mut f64, mut what : *mut f64
) {
    let mut theta : f64;
    theta = mr_Norm(w as (*const f64),3i32);
    *{
         let _old = what;
         what = what.offset(1isize);
         _old
     } = *{
              let _old = w;
              w = w.offset(1isize);
              _old
          } / theta;
    *{
         let _old = what;
         what = what.offset(1isize);
         _old
     } = *{
              let _old = w;
              w = w.offset(1isize);
              _old
          } / theta;
    *what = *w / theta;
    *th = theta;
}

#[no_mangle]
pub unsafe extern fn mr_AxisAng6(
    V : *mut f64, mut th : *mut f64, mut S : *mut f64
) {
    let mut theta : f64;
    theta = mr_Norm(V as (*const f64),3i32);
    if mr_NearZero(theta) != 0 {
        theta = mr_Norm(V.offset(3isize) as (*const f64),3i32);
    }
    mr_VS(V as (*const f64),6i32,1.0f64 / theta,S);
    *th = theta;
}

#[no_mangle]
pub unsafe extern fn mr_MatrixExp3(s : *mut f64, mut d : *mut f64) {
    let mut w : [f64; 3];
    let mut whatb : [f64; 9];
    let mut id : [f64; 9];
    let mut temp1 : [f64; 9];
    let mut temp2 : [f64; 9];
    let mut temp3 : [f64; 9];
    let mut th : f64;
    mr_so3ToVec(s,w.as_mut_ptr());
    th = mr_Norm(w.as_mut_ptr() as (*const f64),3i32);
    if mr_NearZero(th) != 0 {
        mr_Id(d,3i32);
    } else {
        mr_so3xS(s,1.0f64 / th,whatb.as_mut_ptr());
        mr_Id(id.as_mut_ptr(),3i32);
        mr_so3xS(whatb.as_mut_ptr(),sin(th),temp1.as_mut_ptr());
        mr_so3Squared(whatb.as_mut_ptr(),temp2.as_mut_ptr());
        mr_MS(
            temp2.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            1i32 as (f64) - cos(th),
            temp3.as_mut_ptr()
        );
        mr_AddMatrices(
            id.as_mut_ptr() as (*const f64),
            temp1.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            temp2.as_mut_ptr()
        );
        mr_AddMatrices(
            temp2.as_mut_ptr() as (*const f64),
            temp3.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            d
        );
    }
}

#[no_mangle]
pub unsafe extern fn mr_MatrixExp6(
    mut s : *mut f64, mut d : *mut f64
) {
    let mut th : f64;
    let mut diag : [f64; 9];
    let mut m33tempa : [f64; 9];
    let mut m33tempb : [f64; 9];
    let mut m33tempc : [f64; 9];
    let mut wbs : [f64; 9];
    let mut whatb : [f64; 9];
    let mut so3temp : [f64; 9];
    let mut w : [f64; 9];
    let mut Rs : [f64; 9];
    let mut R : *mut f64;
    let mut wb : *mut f64;
    let mut v : *mut f64;
    let mut v3b : *mut f64;
    let mut sp : *mut f64;
    let mut vs : [f64; 3];
    let mut v3a : [f64; 3];
    let mut v3bs : [f64; 3];
    R = Rs.as_mut_ptr();
    wb = wbs.as_mut_ptr();
    v = vs.as_mut_ptr();
    v3b = v3bs.as_mut_ptr();
    sp = s;
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = v;
         v = v.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = v;
         v = v.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *{
         let _old = wb;
         wb = wb.offset(1isize);
         _old
     } = *{
              let _old = s;
              s = s.offset(1isize);
              _old
          };
    *wb = *{
               let _old = s;
               s = s.offset(1isize);
               _old
           };
    *v = *s;
    wb = &mut wbs[0usize] as (*mut f64);
    v = &mut vs[0usize] as (*mut f64);
    s = sp;
    mr_so3ToVec(wb,w.as_mut_ptr());
    th = mr_Norm(w.as_mut_ptr() as (*const f64),3i32);
    if mr_NearZero(th) != 0 {
        mr_Id(d,4i32);
        *d.offset(3isize) = *s.offset(3isize);
        *d.offset(7isize) = *s.offset(7isize);
        *d.offset(11isize) = *s.offset(11isize);
    } else {
        mr_so3xS(wb,1.0f64 / th,whatb.as_mut_ptr());
        mr_MatrixExp3(wb,R);
        mr_IdxS(diag.as_mut_ptr(),3i32,th);
        mr_so3xS(
            whatb.as_mut_ptr(),
            1i32 as (f64) - cos(th),
            so3temp.as_mut_ptr()
        );
        mr_so3Squared(whatb.as_mut_ptr(),m33tempa.as_mut_ptr());
        mr_MS(
            m33tempa.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            th - sin(th),
            m33tempb.as_mut_ptr()
        );
        mr_AddMatrices(
            diag.as_mut_ptr() as (*const f64),
            so3temp.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            m33tempa.as_mut_ptr()
        );
        mr_AddMatrices(
            m33tempa.as_mut_ptr() as (*const f64),
            m33tempb.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            m33tempc.as_mut_ptr()
        );
        mr_MV(
            m33tempc.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            v as (*const f64),
            v3a.as_mut_ptr()
        );
        mr_VS(v3a.as_mut_ptr() as (*const f64),3i32,1.0f64 / th,v3b);
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = v3b;
                  v3b = v3b.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = v3b;
                  v3b = v3b.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *{
                  let _old = R;
                  R = R.offset(1isize);
                  _old
              };
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *R;
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = *v3b;
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = d;
             d = d.offset(1isize);
             _old
         } = 0i32 as (f64);
        *d = 1i32 as (f64);
    }
}

#[no_mangle]
pub unsafe extern fn mr_ScrewToAxis(
    q : *mut f64, s : *mut f64, h : f64, mut S : *mut f64
) {
    let mut sx : f64 = *s.offset(0isize);
    let mut sy : f64 = *s.offset(1isize);
    let mut sz : f64 = *s.offset(2isize);
    let mut qx : f64 = *q.offset(0isize);
    let mut qy : f64 = *q.offset(1isize);
    let mut qz : f64 = *q.offset(2isize);
    *S.offset(0isize) = sx;
    *S.offset(1isize) = sy;
    *S.offset(2isize) = sz;
    *S.offset(3isize) = qy * sz - qz * sy + h * sx;
    *S.offset(4isize) = -qx * sz + qz * sx + h * sy;
    *S.offset(5isize) = qx * sy - qy * sx + h * sz;
}

#[no_mangle]
pub unsafe extern fn mr_MatrixLog3(R : *mut f64, mut wb : *mut f64) {
    let mut negI : [f64; 9];
    let mut m33a : [f64; 9];
    let mut m33b : [f64; 9];
    let mut tr : f64;
    let mut mult : f64;
    let mut temp : f64;
    let mut w0 : f64;
    let mut w1 : f64;
    let mut w2 : f64;
    let mut acosin : f64;
    let mut theta : f64;
    let mut v3 : [f64; 3];
    mr_IdxS(negI.as_mut_ptr(),3i32,-1.0f64);
    mr_AddMatrices(
        R as (*const f64),
        negI.as_mut_ptr() as (*const f64),
        3i32,
        3i32,
        m33a.as_mut_ptr()
    );
    if mr_MatrixIsZero(
           m33a.as_mut_ptr() as (*const f64),
           3i32,
           3i32
       ) != 0 {
        mr_ZeroMatrix(wb,3i32,3i32);
    } else {
        tr = mr_Trace(R as (*const f64),3i32);
        if mr_NearZero(tr + 1i32 as (f64)) != 0 {
            if mr_NearZero(
                   {
                       temp = 1i32 as (f64) + *R.offset(8isize);
                       temp
                   }
               ) == 0 {
                mult = 1i32 as (f64) / sqrt(2i32 as (f64) * temp);
                w0 = *R.offset(2isize);
                w1 = *R.offset(5isize);
                w2 = 1i32 as (f64) + *R.offset(8isize);
            } else if mr_NearZero(
                          {
                              temp = 1i32 as (f64) + *R.offset(4isize);
                              temp
                          }
                      ) == 0 {
                mult = 1i32 as (f64) / sqrt(2i32 as (f64) * temp);
                w0 = *R.offset(1isize);
                w1 = 1i32 as (f64) + *R.offset(4isize);
                w2 = *R.offset(7isize);
            } else {
                mult = 1i32 as (f64) / sqrt(
                                           2i32 as (f64) * (1i32 as (f64) + *R.offset(0isize))
                                       );
                w0 = 1i32 as (f64) + *R.offset(0isize);
                w1 = *R.offset(3isize);
                w2 = *R.offset(6isize);
            }
            v3[0usize] = mult * w0;
            v3[1usize] = mult * w1;
            v3[2usize] = mult * w2;
            mr_VecToso3(v3.as_mut_ptr(),wb);
        } else {
            acosin = 0.5f64 * (tr - 1.0f64);
            if acosin > 1i32 as (f64) {
                acosin = 1i32 as (f64);
            }
            if acosin < -1i32 as (f64) {
                acosin = -1i32 as (f64);
            }
            theta = acos(acosin);
            mr_MS(R as (*const f64),3i32,3i32,-1.0f64,m33a.as_mut_ptr());
            mr_Transpose(
                m33a.as_mut_ptr() as (*const f64),
                3i32,
                3i32,
                m33b.as_mut_ptr()
            );
            mr_AddMatrices(
                R as (*const f64),
                m33b.as_mut_ptr() as (*const f64),
                3i32,
                3i32,
                m33a.as_mut_ptr()
            );
            mr_MS(
                m33a.as_mut_ptr() as (*const f64),
                3i32,
                3i32,
                theta * (1.0f64 / (2.0f64 * sin(theta))),
                wb
            );
        }
    }
}

#[no_mangle]
pub unsafe extern fn mr_MatrixLog6(T : *mut f64, mut se3 : *mut f64) {
    let mut p : [f64; 3];
    let mut w : [f64; 3];
    let mut vs : [f64; 3];
    let mut R : [f64; 9];
    let mut so3s : [f64; 9];
    let mut m33a : [f64; 9];
    let mut m33b : [f64; 9];
    let mut m33c : [f64; 9];
    let mut m33d : [f64; 9];
    let mut th : f64;
    let mut thinv : f64;
    let mut v : *mut f64;
    let mut so3 : *mut f64;
    v = vs.as_mut_ptr();
    so3 = so3s.as_mut_ptr();
    mr_TransToRp(T,R.as_mut_ptr(),p.as_mut_ptr());
    mr_IdxS(m33a.as_mut_ptr(),3i32,-1.0f64);
    mr_AddMatrices(
        R.as_mut_ptr() as (*const f64),
        m33a.as_mut_ptr() as (*const f64),
        3i32,
        3i32,
        m33b.as_mut_ptr()
    );
    if mr_MatrixIsZero(
           m33b.as_mut_ptr() as (*const f64),
           3i32,
           3i32
       ) != 0 {
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *T.offset(3isize);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *T.offset(7isize);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *T.offset(11isize);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *se3 = 0i32 as (f64);
    } else {
        mr_MatrixLog3(R.as_mut_ptr(),so3);
        mr_so3ToVec(so3,w.as_mut_ptr());
        th = mr_Norm(w.as_mut_ptr() as (*const f64),3i32);
        thinv = 1i32 as (f64) / th;
        mr_Id(m33a.as_mut_ptr(),3i32);
        mr_so3xS(so3,-0.5f64,m33b.as_mut_ptr());
        mr_so3Squared(so3,m33c.as_mut_ptr());
        mr_MS(
            m33c.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            thinv * thinv - 0.5f64 * thinv / tan(th / 2.0f64),
            m33d.as_mut_ptr()
        );
        mr_AddMatrices(
            m33a.as_mut_ptr() as (*const f64),
            m33b.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            m33c.as_mut_ptr()
        );
        mr_AddMatrices(
            m33c.as_mut_ptr() as (*const f64),
            m33d.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            m33a.as_mut_ptr()
        );
        mr_MV(
            m33a.as_mut_ptr() as (*const f64),
            3i32,
            3i32,
            p.as_mut_ptr() as (*const f64),
            v
        );
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = v;
                  v = v.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = v;
                  v = v.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *{
                  let _old = so3;
                  so3 = so3.offset(1isize);
                  _old
              };
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *so3;
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = *v;
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *{
             let _old = se3;
             se3 = se3.offset(1isize);
             _old
         } = 0i32 as (f64);
        *se3 = 0i32 as (f64);
    }
}

#[no_mangle]
pub unsafe extern fn mr_FKinBody(
    M : *mut f64,
    mut Blist : *const f64,
    mut th : *const f64,
    n : i32,
    mut T : *mut f64
) {
    let mut i : i32;
    let mut theta : f64;
    let mut V : *mut f64;
    let mut Vs : [f64; 6];
    let mut se3 : [f64; 16];
    let mut SE3temp : [f64; 16];
    let mut SE3 : [f64; 16];
    V = Vs.as_mut_ptr();
    mr_CopyMatrix(M as (*const f64),4i32,4i32,T);
    i = 0i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        theta = *{
                     let _old = th;
                     th = th.offset(1isize);
                     _old
                 };
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Blist;
                  Blist = Blist.offset(1isize);
                  _old
              } * theta;
        V = Vs.as_mut_ptr();
        mr_VecTose3(V,se3.as_mut_ptr());
        mr_MatrixExp6(se3.as_mut_ptr(),SE3temp.as_mut_ptr());
        mr_SE3xSE3(T,SE3temp.as_mut_ptr(),SE3.as_mut_ptr());
        mr_CopyMatrix(SE3.as_mut_ptr() as (*const f64),4i32,4i32,T);
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_FKinSpace(
    M : *mut f64,
    mut Slist : *mut f64,
    mut th : *mut f64,
    n : i32,
    mut T : *mut f64
) {
    let mut i : i32;
    let mut theta : f64;
    let mut V : *mut f64;
    let mut Slistp : *mut f64;
    let mut Vs : [f64; 6];
    let mut se3 : [f64; 16];
    let mut SE3temp : [f64; 16];
    let mut SE3 : [f64; 16];
    V = Vs.as_mut_ptr();
    mr_CopyMatrix(M as (*const f64),4i32,4i32,T);
    i = n - 1i32;
    'loop1: loop {
        if !(i >= 0i32) {
            break;
        }
        theta = *th.offset(i as (isize));
        Slistp = Slist.offset((6i32 * i) as (isize));
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        *{
             let _old = V;
             V = V.offset(1isize);
             _old
         } = *{
                  let _old = Slistp;
                  Slistp = Slistp.offset(1isize);
                  _old
              } * theta;
        V = Vs.as_mut_ptr();
        mr_VecTose3(V,se3.as_mut_ptr());
        mr_MatrixExp6(se3.as_mut_ptr(),SE3temp.as_mut_ptr());
        mr_SE3xSE3(SE3temp.as_mut_ptr(),T,SE3.as_mut_ptr());
        mr_CopyMatrix(SE3.as_mut_ptr() as (*const f64),4i32,4i32,T);
        i = i - 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_JacobianBody(
    mut Blist : *mut f64,
    n : i32,
    mut th : *const f64,
    mut Jb : *mut f64
) {
    let mut i : i32;
    let mut ind : i32;
    let mut T : [f64; 16];
    let mut Ttemp1 : [f64; 16];
    let mut Ttemp2 : [f64; 16];
    let mut Blistp : *mut f64;
    let mut theta : f64;
    let mut Jbp : *mut f64;
    let mut v : [f64; 6];
    let mut Jbi : [f64; 6];
    let mut vbracket : [f64; 16];
    mr_Id(T.as_mut_ptr(),4i32);
    i = n - 1i32;
    Blistp = &mut *Blist.offset((i * 6i32) as (isize)) as (*mut f64);
    *Jb.offset(i as (isize)) = *{
                                    let _old = Blistp;
                                    Blistp = Blistp.offset(1isize);
                                    _old
                                };
    *Jb.offset((i + n) as (isize)) = *{
                                          let _old = Blistp;
                                          Blistp = Blistp.offset(1isize);
                                          _old
                                      };
    *Jb.offset((i + 2i32 * n) as (isize)) = *{
                                                 let _old = Blistp;
                                                 Blistp = Blistp.offset(1isize);
                                                 _old
                                             };
    *Jb.offset((i + 3i32 * n) as (isize)) = *{
                                                 let _old = Blistp;
                                                 Blistp = Blistp.offset(1isize);
                                                 _old
                                             };
    *Jb.offset((i + 4i32 * n) as (isize)) = *{
                                                 let _old = Blistp;
                                                 Blistp = Blistp.offset(1isize);
                                                 _old
                                             };
    *Jb.offset((i + 5i32 * n) as (isize)) = *Blistp;
    i = n - 2i32;
    'loop1: loop {
        if !(i >= 0i32) {
            break;
        }
        ind = i + 1i32;
        theta = *th.offset(ind as (isize));
        Blistp = &mut *Blist.offset((ind * 6i32) as (isize)) as (*mut f64);
        v[0usize] = -*{
                          let _old = Blistp;
                          Blistp = Blistp.offset(1isize);
                          _old
                      } * theta;
        v[1usize] = -*{
                          let _old = Blistp;
                          Blistp = Blistp.offset(1isize);
                          _old
                      } * theta;
        v[2usize] = -*{
                          let _old = Blistp;
                          Blistp = Blistp.offset(1isize);
                          _old
                      } * theta;
        v[3usize] = -*{
                          let _old = Blistp;
                          Blistp = Blistp.offset(1isize);
                          _old
                      } * theta;
        v[4usize] = -*{
                          let _old = Blistp;
                          Blistp = Blistp.offset(1isize);
                          _old
                      } * theta;
        v[5usize] = -*Blistp * theta;
        mr_VecTose3(v.as_mut_ptr(),vbracket.as_mut_ptr());
        mr_MatrixExp6(vbracket.as_mut_ptr(),Ttemp1.as_mut_ptr());
        mr_SE3xSE3(T.as_mut_ptr(),Ttemp1.as_mut_ptr(),Ttemp2.as_mut_ptr());
        mr_CopyMatrix(
            Ttemp2.as_mut_ptr() as (*const f64),
            4i32,
            4i32,
            T.as_mut_ptr()
        );
        Blistp = &mut *Blist.offset((i * 6i32) as (isize)) as (*mut f64);
        v[0usize] = *{
                         let _old = Blistp;
                         Blistp = Blistp.offset(1isize);
                         _old
                     };
        v[1usize] = *{
                         let _old = Blistp;
                         Blistp = Blistp.offset(1isize);
                         _old
                     };
        v[2usize] = *{
                         let _old = Blistp;
                         Blistp = Blistp.offset(1isize);
                         _old
                     };
        v[3usize] = *{
                         let _old = Blistp;
                         Blistp = Blistp.offset(1isize);
                         _old
                     };
        v[4usize] = *{
                         let _old = Blistp;
                         Blistp = Blistp.offset(1isize);
                         _old
                     };
        v[5usize] = *Blistp;
        mr_Adjoint(T.as_mut_ptr(),v.as_mut_ptr(),Jbi.as_mut_ptr());
        Jbp = Jbi.as_mut_ptr();
        *Jb.offset(i as (isize)) = *{
                                        let _old = Jbp;
                                        Jbp = Jbp.offset(1isize);
                                        _old
                                    };
        *Jb.offset((i + n) as (isize)) = *{
                                              let _old = Jbp;
                                              Jbp = Jbp.offset(1isize);
                                              _old
                                          };
        *Jb.offset((i + 2i32 * n) as (isize)) = *{
                                                     let _old = Jbp;
                                                     Jbp = Jbp.offset(1isize);
                                                     _old
                                                 };
        *Jb.offset((i + 3i32 * n) as (isize)) = *{
                                                     let _old = Jbp;
                                                     Jbp = Jbp.offset(1isize);
                                                     _old
                                                 };
        *Jb.offset((i + 4i32 * n) as (isize)) = *{
                                                     let _old = Jbp;
                                                     Jbp = Jbp.offset(1isize);
                                                     _old
                                                 };
        *Jb.offset((i + 5i32 * n) as (isize)) = *Jbp;
        i = i - 1;
    }
}

#[no_mangle]
pub unsafe extern fn mr_JacobianSpace(
    mut Slist : *mut f64,
    n : i32,
    mut th : *const f64,
    mut Js : *mut f64
) {
    let mut i : i32;
    let mut ind : i32;
    let mut T : [f64; 16];
    let mut Ttemp1 : [f64; 16];
    let mut Ttemp2 : [f64; 16];
    let mut Slistp : *mut f64;
    let mut theta : f64;
    let mut Jsp : *mut f64;
    let mut v : [f64; 6];
    let mut Jsi : [f64; 6];
    let mut vbracket : [f64; 16];
    mr_Id(T.as_mut_ptr(),4i32);
    Slistp = Slist;
    *Js.offset(0isize) = *{
                              let _old = Slistp;
                              Slistp = Slistp.offset(1isize);
                              _old
                          };
    *Js.offset(n as (isize)) = *{
                                    let _old = Slistp;
                                    Slistp = Slistp.offset(1isize);
                                    _old
                                };
    *Js.offset((2i32 * n) as (isize)) = *{
                                             let _old = Slistp;
                                             Slistp = Slistp.offset(1isize);
                                             _old
                                         };
    *Js.offset((3i32 * n) as (isize)) = *{
                                             let _old = Slistp;
                                             Slistp = Slistp.offset(1isize);
                                             _old
                                         };
    *Js.offset((4i32 * n) as (isize)) = *{
                                             let _old = Slistp;
                                             Slistp = Slistp.offset(1isize);
                                             _old
                                         };
    *Js.offset((5i32 * n) as (isize)) = *Slistp;
    i = 1i32;
    'loop1: loop {
        if !(i < n) {
            break;
        }
        ind = i - 1i32;
        theta = *th.offset(ind as (isize));
        Slistp = &mut *Slist.offset((ind * 6i32) as (isize)) as (*mut f64);
        v[0usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     } * theta;
        v[1usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     } * theta;
        v[2usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     } * theta;
        v[3usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     } * theta;
        v[4usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     } * theta;
        v[5usize] = *Slistp * theta;
        mr_VecTose3(v.as_mut_ptr(),vbracket.as_mut_ptr());
        mr_MatrixExp6(vbracket.as_mut_ptr(),Ttemp1.as_mut_ptr());
        mr_SE3xSE3(T.as_mut_ptr(),Ttemp1.as_mut_ptr(),Ttemp2.as_mut_ptr());
        mr_CopyMatrix(
            Ttemp2.as_mut_ptr() as (*const f64),
            4i32,
            4i32,
            T.as_mut_ptr()
        );
        Slistp = &mut *Slist.offset((i * 6i32) as (isize)) as (*mut f64);
        v[0usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     };
        v[1usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     };
        v[2usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     };
        v[3usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     };
        v[4usize] = *{
                         let _old = Slistp;
                         Slistp = Slistp.offset(1isize);
                         _old
                     };
        v[5usize] = *Slistp;
        mr_Adjoint(T.as_mut_ptr(),v.as_mut_ptr(),Jsi.as_mut_ptr());
        Jsp = Jsi.as_mut_ptr();
        *Js.offset(i as (isize)) = *{
                                        let _old = Jsp;
                                        Jsp = Jsp.offset(1isize);
                                        _old
                                    };
        *Js.offset((i + n) as (isize)) = *{
                                              let _old = Jsp;
                                              Jsp = Jsp.offset(1isize);
                                              _old
                                          };
        *Js.offset((i + 2i32 * n) as (isize)) = *{
                                                     let _old = Jsp;
                                                     Jsp = Jsp.offset(1isize);
                                                     _old
                                                 };
        *Js.offset((i + 3i32 * n) as (isize)) = *{
                                                     let _old = Jsp;
                                                     Jsp = Jsp.offset(1isize);
                                                     _old
                                                 };
        *Js.offset((i + 4i32 * n) as (isize)) = *{
                                                     let _old = Jsp;
                                                     Jsp = Jsp.offset(1isize);
                                                     _old
                                                 };
        *Js.offset((i + 5i32 * n) as (isize)) = *Jsp;
        i = i + 1;
    }
}
