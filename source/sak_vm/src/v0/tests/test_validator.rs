#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use crate::{CtrFn, VM};
    use env_logger::init;
    use sak_contract_std::{Request, Storage};

    fn get_dummy_validator_1() -> String {
        String::from(
            "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            1111111111111111111111111111111\
            ",
        )
    }
    fn get_dummy_validator_2() -> String {
        String::from(
            "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            2222222222222222222222222222222\
            ",
        )
    }
    fn get_dummy_validator_3() -> String {
        String::from(
            "\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
            ccccccccccccccccccccccccccccccccc\
            3333333333333333333333333333333\
            ",
        )
    }

    fn get_test_validator() -> String {
        String::from(
            "\
            046885b904a8b8cdd17cc40078ed11421\
            4586f197a664d6aa33d4b46cc3b712afc\
            def3d4d808bc7843beaea9e1a4c5ddeea\
            47cbd27ea1af5ca13719a2f42c39167\
            ",
        )
    }

    fn get_test_validator_state(validators_vec: Vec<String>) -> Storage {
        let mut ret = Storage::with_capacity(10);

        let key = String::from("validators");
        let value = serde_json::to_string(&validators_vec).unwrap();

        ret.insert(key, value);

        ret
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_ctr_validator_fn_init() {
        init();
        let vm = VM::init().expect("VM should be initiated");

        let ctr_wasm = include_bytes!("../sak_ctrt_validator.wasm").to_vec();
        let ctr_fn = CtrFn::Init;

        let ctr_validator_init_state = vm
            .exec(ctr_wasm, ctr_fn)
            .expect("validator should be obtained");

        let ctr_validator_state: Storage =
            serde_json::from_str(ctr_validator_init_state.as_str()).unwrap();

        let validator_list_from_ctr: Vec<String> = serde_json::from_str(
            ctr_validator_state.get("validators").unwrap(),
        )
        .unwrap();

        let validator_list_expected = vec![get_test_validator()];

        println!("validator list expected: {:?}", validator_list_expected);
        println!("validator list acquired: {:?}", validator_list_from_ctr);

        assert_eq!(
            //
            validator_list_expected,
            validator_list_from_ctr
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_ctr_validator_fn_query() {
        init();
        /*

            1. get initial storage from db (suppose)
            2. get validator_acquired = exec.query(request, init_storage)
            3. check if validator == validator_acquired

        */
        let vm = VM::init().expect("VM should be initiated");

        let test_validator_vec = vec![
            get_test_validator(),
            get_dummy_validator_1(),
            get_dummy_validator_2(),
            get_dummy_validator_3(),
        ];

        let request = Request {
            req_type: "get_validator".to_string(),
            arg: HashMap::with_capacity(10),
        };
        let storage = get_test_validator_state(test_validator_vec.clone());

        let ctr_wasm = include_bytes!("../sak_ctrt_validator.wasm").to_vec();
        let ctr_fn = CtrFn::Query(request, storage);

        let validator_from_fn_query = vm
            .exec(ctr_wasm, ctr_fn)
            .expect("validator should be obtained");

        assert_eq!(
            //
            test_validator_vec[0],
            validator_from_fn_query
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_validator_ctrt_execute_fn() {
        assert_eq!("exeucte", "exeucte");
    }
}
