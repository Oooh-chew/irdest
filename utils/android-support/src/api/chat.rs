// use crate::utils::{self, GcWrapped, JavaId};
// use async_std::{sync::Arc, task::block_on};
// use jni::{
//     objects::{JClass, JList, JObject, JString, JValue},
//     sys::{jboolean, jint, jlong, jobject},
//     JNIEnv,
// };
// use libqaul::{ffi::java::ToJObject, Identity};
// use qaul_chat::{ChatMessage, RoomMeta};

// fn room_to_jobject<'env>(env: &'env JNIEnv, room: RoomMeta) -> JObject<'env> {
//     let id = JavaId::from_identity(room.id).into_obj(env);
//     let name = utils::into_jstring(env, room.name.unwrap_or("Untitled chat".into()));
//     let last_message = utils::into_jstring(env, "<last mesage>".into());
//     let unread = room.unread as jint;

//     let list_class = env.find_class("java/util/ArrayList").unwrap();
//     let arraylist = env.new_object(list_class, "()V", &[]).unwrap();
//     let list = JList::from_env(env, arraylist).unwrap();

//     let members = room
//         .users
//         .into_iter()
//         .map(|user_id| JavaId::from_identity(user_id).into_obj(env))
//         .fold(list, |list, jobj| {
//             list.add(jobj);
//             list
//         });

//     let room_class: JClass<'env> = env.find_class("net/qaul/app/ffi/models/ChatRoom").unwrap();

//     env.new_object(
//         room_class,
//         "(Lnet/qaul/app/ffi/models/Id;Ljava/lang/String;Ljava/lang/String;ILjava/util/ArrayList)V",
//         &[
//             JValue::Object(id),
//             JValue::Object(*name),
//             JValue::Object(*last_message),
//             JValue::Int(unread),
//             JValue::Object(*members),
//         ],
//     )
//     .unwrap()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatList<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     qaul: jlong,
// ) -> jobject {
//     info!("Rust FFI chatList");
//     let state = GcWrapped::from_ptr(qaul as i64);
//     let w = state.get_inner();
//     let auth = state.get_auth().unwrap();

//     let chat = w.chat();

//     let chat_list = block_on(async {
//         let rooms = chat.rooms(auth).await.unwrap();

//         let class = env.find_class("java/util/ArrayList").unwrap();

//         let arraylist = env.new_object(class, "()V", &[]).unwrap();
//         let list = JList::from_env(&env, arraylist).unwrap();

//         rooms
//             .into_iter()
//             .map(|room| room_to_jobject(&env, room))
//             .fold(list, |list, jobj| {
//                 list.add(jobj);
//                 list
//             })
//     });

//     std::mem::forget(qaul);
//     std::mem::forget(state);

//     // Return the list
//     chat_list.into_inner()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatStart<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     qaul: jlong,
//     name: JString,
//     friends: JList,
// ) -> jobject {
//     let state = GcWrapped::from_ptr(qaul as i64);
//     let w = state.get_inner();
//     let auth = state.get_auth().unwrap();

//     let friends: Vec<Identity> = friends
//         .iter()
//         .unwrap()
//         .map(|id| JavaId::from_obj(&env, id).into_identity())
//         .collect();

//     let chat_name = utils::conv_jstring(&env, name);

//     let room = block_on(async {
//         w.chat()
//             .start_chat(auth.clone(), friends, Some(chat_name))
//             .await
//             .unwrap()
//     });

//     let jroom = room_to_jobject(&env, room);
//     std::mem::forget(qaul);
//     std::mem::forget(state);

//     jroom.into_inner()
// }

// fn chat_message_to_jobject<'env>(env: &'env JNIEnv, msg: ChatMessage) -> JObject<'env> {
//     let id = JavaId::from_identity(msg.id).into_obj(env);
//     let sender = JavaId::from_identity(msg.sender).into_obj(env);
//     let timestamp = utils::into_jstring(env, format!("{}", msg.timestamp));
//     let content = utils::into_jstring(env, msg.content);

//     let class: JClass<'env> = env
//         .find_class("net/qaul/app/ffi/models/ChatMessage")
//         .unwrap();

//     env.new_object(
//         class,
//         "(Lnet/qaul/app/ffi/models/Id;Lnet/qaul/app/ffi/models/Id;Ljava/lang/String;Ljava/lang/String;)V",
//         &[
//             JValue::Object(id),
//             JValue::Object(sender),
//             JValue::Object(*timestamp),
//             JValue::Object(*content),
//         ],
//     )
//     .unwrap()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatSendMessage<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     qaul: jlong,
//     room_id: JObject,
//     content: JString,
// ) -> jobject {
//     let state = GcWrapped::from_ptr(qaul as i64);
//     let w = state.get_inner();
//     let auth = state.get_auth().unwrap();

//     let room_id = JavaId::from_obj(&env, room_id).into_identity();
//     let content = utils::conv_jstring(&env, content);

//     let chat_message =
//         block_on(async { w.chat().send_message(auth, room_id, content).await.unwrap() });

//     chat_message_to_jobject(&env, chat_message).into_inner()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatLoadMessages<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     qaul: jlong,
//     room_id: JObject,
// ) -> jobject {
//     let state = GcWrapped::from_ptr(qaul as i64);
//     let w = state.get_inner();
//     let auth = state.get_auth().unwrap();

//     let room_id = JavaId::from_obj(&env, room_id).into_identity();

//     let messages = block_on(async { w.chat().load_messages(auth, room_id).await.unwrap() });

//     let list_class = env.find_class("java/util/ArrayList").unwrap();
//     let arraylist = env.new_object(list_class, "()V", &[]).unwrap();
//     let list = JList::from_env(&env, arraylist).unwrap();

//     messages
//         .into_iter()
//         .map(|message| chat_message_to_jobject(&env, message))
//         .fold(list, |list, jobj| {
//             list.add(jobj);
//             list
//         })
//         .into_inner()
// }
