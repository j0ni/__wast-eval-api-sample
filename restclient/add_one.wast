(module
 (type $t0 (func (param i32) (result i32)))
 (func $http_coeffect (import "env" "http_coeffect") (param i32) (result i32))
 (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
       (call $http_coeffect (local.get $p0))))
