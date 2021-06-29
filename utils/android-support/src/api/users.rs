//! Users API scope
//!
//! This API is responsible for creating, listing and managing local
//! and remote users.

use crate::utils::{self, GcWrapped, JavaId};
use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, jlong, jobject},
    JNIEnv,
};

use irdest_core::{users::UserAuth, Irdest};
use ratman_configure::{EpBuilder, NetBuilder};

/// Lits all local users that are available for login
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersList(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
    local: jboolean,
) -> jobject {
    info!("Rust FFI usersList");
    let state = GcWrapped::from_ptr(qaul as i64);
    let w = state.get_inner();
    let obj = (*irdest_core::ffi::java::users::list(local, &env, w.qaul())).into_inner();
    std::mem::forget(state);
    obj
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersCreate<'env>(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
    handle: JString,
    name: JString,
    pw: JString,
) -> jobject {
    info!("Rust FFI usersCreate");
    let state = GcWrapped::from_ptr(qaul as i64);
    let w = state.get_inner();

    match irdest_core::ffi::java::users::create(&env, w.qaul(), handle, name, pw) {
        Err(e) => {
            error!("Error occured while creating user: {:?}", e);
            std::mem::forget(state); // FIXME
            *JObject::null()
        }
        Ok(auth) => {
            info!("6");
            let id = auth.0;
            info!("7");
            state.set_auth(Some(auth));
            info!("8");
            std::mem::forget(state); // FIXME
            JavaId::from_identity(id).into_obj(&env).into_inner()
        }
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersModify<'env>(
//     env: JNIEnv,
//     _: JObject,
//     qaul: jlong,
//     handle: JString,
//     name: JString,
// ) -> jobject {
//     info!("Rust FFI usersModify");
//     let state = GcWrapped::from_ptr(qaul as i64);
//     let auth = state.get_auth().unwrap();
//     let w = state.get_inner();

//     let handle = utils::maybe_conv_jstring(&env, handle);
//     let name = utils::maybe_conv_jstring(&env, name);

//     block_on(async {
//         use irdest_core::users::UserUpdate;
//         let updates = vec![UserUpdate::DisplayName(handle), UserUpdate::RealName(name)];

//         for u in updates {
//             match w.qaul().users().update(auth.clone(), u).await {
//                 Ok(_) => continue,
//                 Err(e) => error!("Failure: {}", e), // TODO: return proper failure?
//             }
//         }

//         irdest_core::ffi::java::users::get(&env, w.qaul(), auth.0).into_inner()
//     })
// }

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersLogin(
    env: JNIEnv,
    _this: JObject,
    qaul: jlong,
    id: JObject,
    pw: JString,
) -> jboolean {
    info!("Rust FFI usersLogin");
    let state = GcWrapped::from_ptr(qaul as i64);
    let w = state.get_inner();

    let id = JavaId::from_obj(&env, id).into_identity();

    let b = (match irdest_core::ffi::java::users::login(&env, w.qaul(), id, pw) {
        Ok(auth) => {
            state.set_auth(Some(auth));
            true
        }
        Err(_) => false,
    }) as jboolean;

    std::mem::forget(w);
    std::mem::forget(state);

    b
}
