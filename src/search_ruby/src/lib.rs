use rutie::{Class, class, methods, Object, RString, VM};

use harana_transfer::transfer::Transfer;

class!(HaranaSearch);

methods!(
    HaranaSearch,
    _itself,

    fn add_documents(index_path: RString, index_declarations_path: RString, index_name: RString, document_payloads_json: RString) -> RString {
        let index_path: String = index_path.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let index_declarations_path: String = index_declarations_path.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let index_name: String = index_name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let document_payloads_json: String = document_payloads_json.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        Transfer::from_external(index_path, index_declarations_path, index_name, document_payloads_json)
        RString::new_utf8("")
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn init() {
    Class::new("HaranaSearch", None).define(|this| {
        this.def_self("add_documents", add_documents);
    });
}
