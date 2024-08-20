use jni::JNIEnv;
use jni::objects::{JClass, JString};
use harana_transfer::transfer::Transfer;

#[no_mangle]
pub extern "system" fn Java_Search_add_documents<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    index_path: JString<'local>,
    index_declarations_path: JString<'local>,
    index_name: JString<'local>,
    document_payloads_json: JString<'local>
) {
    let index_path = String::from(env.get_string(&index_path).expect("Couldn't get index_path"));
    let index_declarations_path = String::from(env.get_string(&index_declarations_path).expect("Couldn't get index_declarations_path"));
    let index_name = String::from(env.get_string(&index_name).expect("Couldn't get index_name"));
    let document_payloads_json = String::from(env.get_string(&document_payloads_json).expect("Couldn't get document_payloads"));

    Transfer::from_external(index_path, index_declarations_path, index_name, document_payloads_json)
}