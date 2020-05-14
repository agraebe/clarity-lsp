use super::functions::{NativeFunctions, NativeVariables, DefineFunctions};
use super::types::{FunctionType, FixedFunction};
use super::analysis::type_checker::{TypedNativeFunction};
use super::analysis::type_checker::natives::SimpleNativeFunction;

use serde::{Serialize};

#[derive(Serialize)]
struct ReferenceAPIs {
    functions: Vec<FunctionAPI>,
    keywords: Vec<KeywordAPI>
}

#[derive(Serialize, Clone)]
pub struct KeywordAPI {
    pub name: &'static str,
    pub snippet: &'static str,
    pub output_type: &'static str,
    pub description: &'static str,
    pub example: &'static str
}

#[derive(Serialize)]
pub struct FunctionAPI {
    pub name: String,
    pub snippet: String,
    pub input_type: String,
    pub output_type: String,
    pub signature: String,
    pub description: String,
    pub example: String
}

pub struct SimpleFunctionAPI {
    name: Option<&'static str>,
    snippet: &'static str,
    signature: &'static str,
    description: &'static str,
    example: &'static str,
}

struct SpecialAPI {
    output_type: &'static str,
    snippet: &'static str,
    input_type: &'static str,
    signature: &'static str,
    description: &'static str,
    example: &'static str,
}

pub struct DefineAPI {
    output_type: &'static str,
    snippet: &'static str,
    input_type: &'static str,
    signature: &'static str,
    description: &'static str,
    example: &'static str,
}

const BLOCK_HEIGHT: KeywordAPI = KeywordAPI {
    name: "block-height",
    snippet: "block-height",
    output_type: "uint",
    description: "Returns the current block height of the Stacks blockchain as an int",
    example: "(> block-height 1000) ;; returns true if the current block-height has passed 1000 blocks."
};

const BURN_BLOCK_HEIGHT: KeywordAPI = KeywordAPI {
    name: "burn-block-height",
    snippet: "burn-block-height",
    output_type: "uint",
    description: "Returns the current block height of the Stacks blockchain as an int",
    example: "(> block-height 1000) ;; returns true if the current block-height has passed 1000 blocks."
};

const CONTRACT_CALLER_KEYWORD: KeywordAPI = KeywordAPI {
    name: "contract-caller",
    snippet: "contract-caller",
    output_type: "principal",
    description: "Returns the caller of the current contract context. If this contract is the first one called by a signed transaction,
the caller will be equal to the signing principal. If `contract-call?` was used to invoke a function from a new contract, `contract-caller`
changes to the _calling_ contract's principal. If `as-contract` is used to change the `tx-sender` context, `contract-caller` _also_ changes
to the same contract principal.",
    example: "(print contract-caller) ;; Will print out a Stacks address of the transaction sender",
};

const TX_SENDER_KEYWORD: KeywordAPI = KeywordAPI {
    name: "tx-sender",
    snippet: "tx-sender",
    output_type: "principal",
    description: "Returns the original sender of the current transaction, or if `as-contract` was called to modify the sending context, it returns that
contract principal.",
    example: "(print tx-sender) ;; Will print out a Stacks address of the transaction sender",
};

const NONE_KEYWORD: KeywordAPI = KeywordAPI {
    name: "none",
    snippet: "none",
    output_type: "(optional ?)",
    description: "Represents the _none_ option indicating no value for a given optional (analogous to a null value).",
    example: "
(define-public (only-if-positive (a int))
  (if (> a 0)
      (some a)
      none))
(only-if-positive 4) ;; Returns (some 4)
(only-if-positive (- 3)) ;; Returns none
"
};

const TRUE_KEYWORD: KeywordAPI = KeywordAPI {
    name: "true",
    snippet: "true",
    output_type: "bool",
    description: "Boolean true constant.",
    example: "
(and true false) ;; Evaluates to false
(or false true)  ;; Evaluates to true
"
};

const FALSE_KEYWORD: KeywordAPI = KeywordAPI {
    name: "false",
    snippet: "false",
    output_type: "bool",
    description: "Boolean false constant.",
    example: "
(and true false) ;; Evaluates to false
(or false true)  ;; Evaluates to true
"
};


const TO_UINT_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(to-uint ${1:int})",
    signature: "(to-uint i)",
    description: "Tries to convert the `int` argument to a `uint`. Will cause a runtime error and abort if the supplied argument is negative.",
    example: "(to-uint 238) ;; Returns u238"
};

const TO_INT_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "none",
    signature: "(to-int u)",
    description: "Tries to convert the `uint` argument to an `int`. Will cause a runtime error and abort if the supplied argument is >= `pow(2, 127)`",
    example: "(to-int u238) ;; Returns 238"
};

const ADD_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("+ (add)"),
    snippet: "(+ ${1:expr-1} ${2:expr-2})",
    signature: "(+ i1 i2...)",
    description: "Adds a variable number of integer inputs and returns the result. In the event of an _overflow_, throws a runtime error.",
    example: "(+ 1 2 3) ;; Returns 6"
};

const SUB_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("- (subtract)"),
    snippet: "(- ${1:expr-1} ${2:expr-2})",
    signature: "(- i1 i2...)",
    description: "Subtracts a variable number of integer inputs and returns the result. In the event of an _underflow_, throws a runtime error.",
    example: "(- 2 1 1) ;; Returns 0
(- 0 3) ;; Returns -3
"
};

const DIV_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("/ (divide)"),
    snippet: "(/ ${1:expr-1} ${2:expr-2})",
    signature: "(/ i1 i2...)",
    description: "Integer divides a variable number of integer inputs and returns the result. In the event of division by zero, throws a runtime error.",
    example: "(/ 2 3) ;; Returns 0
(/ 5 2) ;; Returns 2
(/ 4 2 2) ;; Returns 1
"
};

const MUL_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("* (multiply)"),
    snippet: "(* ${1:expr-1} ${2:expr-2})",
    signature: "(* i1 i2...)",
    description: "Multiplies a variable number of integer inputs and returns the result. In the event of an _overflow_, throws a runtime error.",
    example: "(* 2 3) ;; Returns 6
(* 5 2) ;; Returns 10
(* 2 2 2) ;; Returns 8
"
};

const MOD_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(mod ${1:expr-1} ${2:expr-2})",
    signature: "(mod i1 i2)",
    description: "Returns the integer remainder from integer dividing `i1` by `i2`. In the event of a division by zero, throws a runtime error.",
    example: "(mod 2 3) ;; Returns 0
(mod 5 2) ;; Returns 1
(mod 7 1) ;; Returns 0
"
};

const POW_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(pow ${1:expr-1} ${2:expr-2})",
    signature: "(pow i1 i2)",
    description: "Returns the result of raising `i1` to the power of `i2`. In the event of an _overflow_, throws a runtime error.",
    example: "(pow 2 3) ;; Returns 8
(pow 2 2) ;; Returns 4
(pow 7 1) ;; Returns 7
"
};

const XOR_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(xor ${1:expr-1} ${2:expr-2})",
    signature: "(xor i1 i2)",
    description: "Returns the result of bitwise exclusive or'ing `i1` with `i2`.",
    example: "(xor 1 2) ;; Returns 3
(xor 120 280) ;; Returns 352
"
};

const AND_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(and ${1:expr-1} ${2:expr-2})",
    signature: "(and b1 b2 ...)",
    description: "Returns `true` if all boolean inputs are `true`. Importantly, the supplied arguments are evaluated in-order and lazily. Lazy evaluation means that if one of the arguments returns `false`, the function short-circuits, and no subsequent arguments are evaluated.",
    example: "(and true false) ;; Returns false
(and (is-eq (+ 1 2) 1) (is-eq 4 4)) ;; Returns false
(and (is-eq (+ 1 2) 3) (is-eq 4 4)) ;; Returns true
"
};

const OR_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(or ${1:expr-1} ${2:expr-2})",
    signature: "(or b1 b2 ...)",
    description: "Returns `true` if any boolean inputs are `true`. Importantly, the supplied arguments are evaluated in-order and lazily. Lazy evaluation means that if one of the arguments returns `false`, the function short-circuits, and no subsequent arguments are evaluated.",
    example: "(or true false) ;; Returns true
(or (is-eq (+ 1 2) 1) (is-eq 4 4)) ;; Returns true
(or (is-eq (+ 1 2) 1) (is-eq 3 4)) ;; Returns false
(or (is-eq (+ 1 2) 3) (is-eq 4 4)) ;; Returns true
"
};

const NOT_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(not ${1:expr-1})",
    signature: "(not b1)",
    description: "Returns the inverse of the boolean input.",
    example: "(not true) ;; Returns false
(not (is-eq 1 2)) ;; Returns true
"
};

const GEQ_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some(">= (greater than or equal)"),
    snippet: "(>= ${1:expr-1} ${2:expr-2})",
    signature: "(>= i1 i2)",
    description: "Compares two integers, returning `true` if `i1` is greater than or equal to `i2` and `false` otherwise.",
    example: "(>= 1 1) ;; Returns true
(>= 5 2) ;; Returns true
"
};

const LEQ_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("<= (less than or equal)"),
    snippet: "(<= ${1:expr-1} ${2:expr-2})",
    signature: "(<= i1 i2)",
    description: "Compares two integers, returning true if `i1` is less than or equal to `i2` and `false` otherwise.",
    example: "(<= 1 1) ;; Returns true
(<= 5 2) ;; Returns false
"
};

const GREATER_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("> (greater than)"),
    snippet: "(> ${1:expr-1} ${2:expr-2})",
    signature: "(> i1 i2)",
    description: "Compares two integers, returning `true` if `i1` is greater than `i2` and false otherwise.",
    example: "(> 1 2) ;; Returns false
(> 5 2) ;; Returns true
"
};

const LESS_API: SimpleFunctionAPI = SimpleFunctionAPI {
    name: Some("< (less than)"),
    snippet: "(< ${1:expr-1} ${2:expr-2})",
    signature: "(< i1 i2)",
    description: "Compares two integers, returning `true` if `i1` is less than `i2` and `false` otherwise.",
    example: "(< 1 2) ;; Returns true
(< 5 2) ;; Returns false
"
};

fn make_for_simple_native(api: &SimpleFunctionAPI, function: &NativeFunctions, name: String) -> FunctionAPI {
    let (input_type, output_type) = {
        if let TypedNativeFunction::Simple(SimpleNativeFunction(function_type)) = TypedNativeFunction::type_native_function(&function) {
            let input_type = match function_type {
                FunctionType::Variadic(ref in_type, _) => {
                    format!("{}, ...", in_type)
                },
                FunctionType::Fixed(FixedFunction{ ref args, .. }) => {
                    let in_types: Vec<String> = args.iter().map(|x| format!("{}", x.signature)).collect();
                    in_types.join(", ")
                },
                FunctionType::UnionArgs(ref in_types, _) => {
                    let in_types: Vec<String> = in_types.iter().map(|x| format!("{}", x)).collect();
                    in_types.join(" | ")
                },
                FunctionType::ArithmeticVariadic => "int, ... | uint, ...".to_string(),
                FunctionType::ArithmeticBinary | FunctionType::ArithmeticComparison => "int, int | uint, uint".to_string(),
            };
            let output_type = match function_type {
                FunctionType::Variadic(_, ref out_type) => format!("{}", out_type),
                FunctionType::Fixed(FixedFunction{ ref returns, .. }) => format!("{}", returns),
                FunctionType::UnionArgs(_, ref out_type) => format!("{}", out_type),
                FunctionType::ArithmeticVariadic | FunctionType::ArithmeticBinary => "int | uint".to_string(),
                FunctionType::ArithmeticComparison => "bool".to_string(),
            };
            (input_type, output_type)
        } else {
            panic!("Attempted to auto-generate docs for non-simple native function: {:?}", api.name)
        }
    };

    FunctionAPI {
        name: api.name.map_or(name, |x| x.to_string()),
        snippet: api.snippet.to_string(),
        input_type: input_type,
        output_type: output_type,
        signature: api.signature.to_string(),
        description: api.description.to_string(),
        example: api.example.to_string()
    }
}

const EQUALS_API: SpecialAPI = SpecialAPI {
    input_type: "A, A, ...",
    snippet: "(is-eq ${1:expr-1} ${2:expr-2})",
    output_type: "bool",
    signature: "(is-eq v1 v2...)",
    description: "Compares the inputted values, returning `true` if they are all equal. Note that _unlike_ the `(and ...)` function, `(is-eq ...)` will _not_ short-circuit.",
    example: "(is-eq 1 1) ;; Returns true
(is-eq 1 false) ;; Returns false
(is-eq \"abc\" 234 234) ;; Returns false
"
};

const IF_API: SpecialAPI = SpecialAPI {
    input_type: "bool, A, A",
    snippet: "(if ${1:condition} ${2:expr-if-true} ${3:expr-if-false})",
    output_type: "A",
    signature: "(if bool1 expr1 expr2)",
    description: "The `if` function admits a boolean argument and two expressions
which must return the same type. In the case that the boolean input is `true`, the
`if` function evaluates and returns `expr1`. If the boolean input is `false`, the
`if` function evaluates and returns `expr2`.",
    example: "(if true 1 2) ;; Returns 1
(if (> 1 2) 1 2) ;; Returns 2"
};

const LET_API: SpecialAPI = SpecialAPI {
    input_type: "((name2 AnyType) (name2 AnyType) ...), AnyType, ... A",
    snippet: "(let ((${1:name-1} ${2:val-1})) ${3:expr-1})",
    output_type: "A",
    signature: "(let ((name1 expr1) (name2 expr2) ...) expr-body1 expr-body2 ... expr-body-last)",
    description: "The `let` function accepts a list of `variable name` and `expression` pairs,
evaluating each expression and _binding_ it to the corresponding variable name. The _context_
created by this set of bindings is used for evaluating its body expressions. The let expression returns the value of the last such body expression.",
    example: "(let ((a 2) (b (+ 5 6 7))) (print a) (print b) (+ a b)) ;; Returns 20"
};

const FETCH_VAR_API: SpecialAPI = SpecialAPI {
    input_type: "VarName",
    snippet: "(var-get ${1:var-name})",
    output_type: "A",
    signature: "(var-get var-name)",
    description: "The `var-get` function looks up and returns an entry from a contract's data map.
The value is looked up using `var-name`.",
    example: "(var-get cursor) ;; Returns cursor"
};

const SET_VAR_API: SpecialAPI = SpecialAPI {
    input_type: "VarName, AnyType",
    snippet: "(var-set ${1:var-name} ${2:value})",
    output_type: "bool",
    signature: "(var-set var-name expr1)",
    description: "The `var-set` function sets the value associated with the input variable to the
inputted value.",
    example: "(var-set cursor (+ cursor 1)) ;; Returns true"
};

const MAP_API: SpecialAPI = SpecialAPI {
    input_type: "Function(A) -> B, (list A)",
    snippet: "(map ${1:func} ${2:list})",
    output_type: "(list B)",
    signature: "(map func list)",
    description: "The `map` function applies the input function `func` to each element of the
input list, and outputs a list containing the _outputs_ from those function applications.",
    example: "(map not (list true false true false)) ;; Returns false true false true"
};

const FILTER_API: SpecialAPI = SpecialAPI {
    input_type: "Function(A) -> bool, (list A)",
    snippet: "(filter ${1:func} ${2:list})",
    output_type: "(list A)",
    signature: "(filter func list)",
    description: "The `filter` function applies the input function `func` to each element of the
input list, and returns the same list with any elements removed for which the `func` returned `false`.",
    example: "(filter not (list true false true false)) ;; Returns (list false false)"
};

const FOLD_API: SpecialAPI = SpecialAPI {
    input_type: "Function(A, B) -> B, (list A), B",
    snippet: "(fold ${1:func} ${2:list} ${2:initial-value})",
    output_type: "B",
    signature: "(fold func list initial-value)",
    description: "The `fold` special form applies the input function `func` to each element of the
input list _and_ the output of the previous application of the `fold` function. When invoked on
the first list element, it uses the `initial-value` as the second input. `fold` returns the last
value returned by the successive applications. Note that the first argument is not evaluated thus
has to be a literal function name.",
    example: "(fold * (list 2 2 2) 1) ;; Returns 8
(fold * (list 2 2 2) 0) ;; Returns 0
(fold - (list 3 7 11) 2) ;; Returns 5 by calculating (- 11 (- 7 (- 3 2)))
(fold concat \"cdef\" \"ab\")   ;; Returns \"fedcab\"
(fold concat (list \"cd\" \"ef\") \"ab\")   ;; Returns \"efcdab\""
};

const CONCAT_API: SpecialAPI = SpecialAPI {
    input_type: "(buff, buff)|(list, list)",
    snippet: "(concat ${1:iterable-1} ${2:iterable-2})",
    output_type: "buff|list",
    signature: "(concat buff-a buff-b)",
    description: "The `concat` function takes two buffers or two lists with the same entry type,
and returns a concatenated buffer or list of the same entry type, with max_len = max_len_a + max_len_b.",
    example: "(concat \"hello \" \"world\") ;; Returns \"hello world\""
};

const APPEND_API: SpecialAPI = SpecialAPI {
    input_type: "list A, A",
    snippet: "(concat ${1:list} ${2:element})",
    output_type: "list",
    signature: "(append (list 1 2 3 4) 5)",
    description: "The `append` function takes a list and another value with the same entry type,
or a buffer and another buffer of length 1 and outputs a buffer or a list of the same type with max_len += 1.",
    example: "(append (list 1 2 3 4) 5) ;; Returns (list 1 2 3 4 5)"
};

const ASSERTS_MAX_LEN_API: SpecialAPI = SpecialAPI {
    input_type: "buff|list, uint",
    snippet: "(as-max-len? ${1:buffer} ${2:max-len})",
    output_type: "(optional buff|list)",
    signature: "(as-max-len? buffer 10)",
    description: "The `as-max-len?` function takes a length N (must be a literal) and a buffer or list argument, which must be typed as a list
or buffer of length M and outputs that same list or buffer, but typed with max length N.
At runtime, a check is performed, which if it fails, returns a (none) option.",
    example: "(as-max-len? (list 2 2 2) 3) ;; Returns (some (list 2 2 2))"
};

const LEN_API: SpecialAPI = SpecialAPI {
    input_type: "buff|list",
    snippet: "(len ${1:iterable})",
    output_type: "uint",
    signature: "(len buffer)",
    description: "The `len` function returns the length of a given buffer or list.",
    example: "(len \"blockstack\") ;; Returns 10
(len (list 1 2 3 4 5)) ;; Returns 5
"
};

const LIST_API: SpecialAPI = SpecialAPI {
    input_type: "A, ...",
    snippet: "(list ${1:expr-1} ${2:expr-2})",
    output_type: "(list A)",
    signature: "(list expr1 expr2 expr3 ...)",
    description: "The `list` function constructs a list composed of the inputted values. Each
supplied value must be of the same type.",
    example: "(list (+ 1 2) 4 5) ;; Returns [3 4 5]",
};

const BEGIN_API: SpecialAPI = SpecialAPI {
    input_type: "AnyType, ... A",
    snippet: "(begin ${1:expr-1} ${2:expr-2})",
    output_type: "A",
    signature: "(begin expr1 expr2 expr3 ... expr-last)",
    description: "The `begin` function evaluates each of its input expressions, returning the
return value of the last such expression.",
    example: "(begin (+ 1 2) 4 5) ;; Returns 5",
};

const PRINT_API: SpecialAPI = SpecialAPI {
    input_type: "A",
    snippet: "(print ${1:expr})",
    output_type: "A",
    signature: "(print expr)",
    description: "The `print` function evaluates and returns its input expression. On Blockstack Core
nodes configured for development (as opposed to production mining nodes), this function prints the resulting value to `STDOUT` (standard output).",
    example: "(print (+ 1 2 3)) ;; Returns 6",
};

const FETCH_ENTRY_API: SpecialAPI = SpecialAPI {
    input_type: "MapName, tuple",
    snippet: "(map-get? ${1:map-name} ${2:key-tuple})",
    output_type: "(optional (tuple))",
    signature: "(map-get? map-name key-tuple)",
    description: "The `map-get?` function looks up and returns an entry from a contract's data map.
The value is looked up using `key-tuple`.
If there is no value associated with that key in the data map, the function returns a `none` option. Otherwise,
it returns `(some value)`.",
    example: "(unwrap-panic (map-get? names-map (tuple (name \"blockstack\")))) ;; Returns (tuple (id 1337))
(unwrap-panic (map-get? names-map ((name \"blockstack\")))) ;; Same command, using a shorthand for constructing the tuple
",
};

const SET_ENTRY_API: SpecialAPI = SpecialAPI {
    input_type: "MapName, tuple_A, tuple_B",
    snippet: "(map-set ${1:map-name} ${2:key-tuple} ${3:value-tuple})",
    output_type: "bool",
    signature: "(map-set map-name key-tuple value-tuple)",
    description: "The `map-set` function sets the value associated with the input key to the
inputted value. This function performs a _blind_ update; whether or not a value is already associated
with the key, the function overwrites that existing association.

Note: the `value-tuple` requires 1 additional byte for storage in the materialized blockchain state,
and therefore the maximum size of a value that may be inserted into a map is MAX_CLARITY_VALUE - 1.",
    example: "(map-set names-map (tuple (name \"blockstack\")) (tuple (id 1337))) ;; Returns true
(map-set names-map ((name \"blockstack\")) ((id 1337))) ;; Same command, using a shorthand for constructing the tuple
",
};

const INSERT_ENTRY_API: SpecialAPI = SpecialAPI {
    input_type: "MapName, tuple_A, tuple_B",
    snippet: "(map-insert ${1:map-name} ${2:key-tuple} ${3:value-tuple})",
    output_type: "bool",
    signature: "(map-insert map-name key-tuple value-tuple)",
    description: "The `map-insert` function sets the value associated with the input key to the
inputted value if and only if there is not already a value associated with the key in the map.
If an insert occurs, the function returns `true`. If a value already existed for
this key in the data map, the function returns `false`.

Note: the `value-tuple` requires 1 additional byte for storage in the materialized blockchain state,
and therefore the maximum size of a value that may be inserted into a map is MAX_CLARITY_VALUE - 1.",
    example: "(map-insert names-map (tuple (name \"blockstack\")) (tuple (id 1337))) ;; Returns true
(map-insert names-map (tuple (name \"blockstack\")) (tuple (id 1337))) ;; Returns false
(map-insert names-map ((name \"blockstack\")) ((id 1337))) ;; Same command, using a shorthand for constructing the tuple
",
};

const DELETE_ENTRY_API: SpecialAPI = SpecialAPI {
    input_type: "MapName, tuple",
    snippet: "(map-delete ${1:map-name} ${2:key-tuple})",
    output_type: "bool",
    signature: "(map-delete map-name key-tuple)",
    description: "The `map-delete` function removes the value associated with the input key for
the given map. If an item exists and is removed, the function returns `true`.
If a value did not exist for this key in the data map, the function returns `false`.",
    example: "(map-delete names-map (tuple (name \"blockstack\"))) ;; Returns true
(map-delete names-map (tuple (name \"blockstack\"))) ;; Returns false
(map-delete names-map ((name \"blockstack\"))) ;; Same command, using a shorthand for constructing the tuple
",
};

const TUPLE_CONS_API: SpecialAPI = SpecialAPI {
    input_type: "(key-name A), (key-name-2 B), ...",
    snippet: "(tuple (${1:key-1} ${2:val-1}))",
    output_type: "(tuple (key-name A) (key-name-2 B) ...)",
    signature: "(tuple ((key0 expr0) (key1 expr1) ...))",
    description: "The `tuple` function constructs a typed tuple from the supplied key and expression pairs.
A `get` function can use typed tuples as input to select specific values from a given tuple.
Key names may not appear multiple times in the same tuple definition. Supplied expressions are evaluated and
associated with the expressions' paired key name.",
    example: "(tuple (name \"blockstack\") (id 1337))"
};

const TUPLE_GET_API: SpecialAPI = SpecialAPI {
    input_type: "KeyName, (tuple) | (optional (tuple))",
    snippet: "(get ${1:key-name} ${2:tuple})",
    output_type: "A",
    signature: "(get key-name tuple)",
    description: "The `get` function fetches the value associated with a given key from the supplied typed tuple.
If an `Optional` value is supplied as the inputted tuple, `get` returns an `Optional` type of the specified key in
the tuple. If the supplied option is a `(none)` option, get returns `(none)`.",
    example: "(get id (tuple (name \"blockstack\") (id 1337))) ;; Returns 1337
(get id (map-get? names-map (tuple (name \"blockstack\")))) ;; Returns (some 1337)
(get id (map-get? names-map (tuple (name \"non-existent\")))) ;; Returns (none)
"
};

const HASH160_API: SpecialAPI = SpecialAPI {
    input_type: "buff|uint|int",
    snippet: "(hash160 ${1:buff})",
    output_type: "(buff 20)",
    signature: "(hash160 value)",
    description: "The `hash160` function computes `RIPEMD160(SHA256(x))` of the inputted value.
If an integer (128 bit) is supplied the hash is computed over the little-endian representation of the
integer.",
    example: "(hash160 0) ;; Returns 0xe4352f72356db555721651aa612e00379167b30f"
};

const SHA256_API: SpecialAPI = SpecialAPI {
    input_type: "buff|uint|int",
    snippet: "(sha256 ${1:buff})",
    output_type: "(buff 32)",
    signature: "(sha256 value)",
    description: "The `sha256` function computes `SHA256(x)` of the inputted value.
If an integer (128 bit) is supplied the hash is computed over the little-endian representation of the
integer.",
    example: "(sha256 0) ;; Returns 0x374708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb"
};

const SHA512_API: SpecialAPI = SpecialAPI {
    input_type: "buff|uint|int",
    snippet: "(sha512 ${1:buff})",
    output_type: "(buff 64)",
    signature: "(sha512 value)",
    description: "The `sha512` function computes `SHA512(x)` of the inputted value.
If an integer (128 bit) is supplied the hash is computed over the little-endian representation of the
integer.",
    example: "(sha512 1) ;; Returns 0x6fcee9a7b7a7b821d241c03c82377928bc6882e7a08c78a4221199bfa220cdc55212273018ee613317c8293bb8d1ce08d1e017508e94e06ab85a734c99c7cc34",
};

const SHA512T256_API: SpecialAPI = SpecialAPI {
    input_type: "buff|uint|int",
    snippet: "(sha512/256 ${1:buff})",
    output_type: "(buff 32)",
    signature: "(sha512/256 value)",
    description: "The `sha512/256` function computes `SHA512/256(x)` (the SHA512 algorithm with the 512/256 initialization vector, truncated
to 256 bits) of the inputted value.
If an integer (128 bit) is supplied the hash is computed over the little-endian representation of the
integer.",
    example: "(sha512/256 1) ;; Returns 0x515a7e92e7c60522db968d81ff70b80818fc17aeabbec36baf0dda2812e94a86",
};

const KECCAK256_API: SpecialAPI = SpecialAPI {
    input_type: "buff|uint|int",
    snippet: "(keccak256 ${1:buff})",
    output_type: "(buff 32)",
    signature: "(keccak256 value)",
    description: "The `keccak256` function computes `KECCAK256(value)` of the inputted value.
Note that this differs from the `NIST SHA-3` (that is, FIPS 202) standard. If an integer (128 bit)
is supplied the hash is computed over the little-endian representation of the integer.",
    example: "(keccak256 0) ;; Returns 0xf490de2920c8a35fabeb13208852aa28c76f9be9b03a4dd2b3c075f7a26923b4"
};

const CONTRACT_CALL_API: SpecialAPI = SpecialAPI {
    input_type: "ContractName, PublicFunctionName, Arg0, ...",
    snippet: "(contract-call? ${1:contract-principal} ${2:func} ${3:arg1})",
    output_type: "(response A B)",
    signature: "(contract-call? .contract-name function-name arg0 arg1 ...)",
    description: "The `contract-call?` function executes the given public function of the given contract.
You _may not_ this function to call a public function defined in the current contract. If the public
function returns _err_, any database changes resulting from calling `contract-call?` are aborted.
If the function returns _ok_, database changes occurred.",
    example: "(contract-call? .tokens transfer 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR 19) ;; Returns (ok 1)"
};

const AT_BLOCK: SpecialAPI = SpecialAPI {
    input_type: "(buff 32), A",
    snippet: "(at-block ${1:id-header-hash} ${2:expr})",
    output_type: "A",
    signature: "(at-block block-hash expr)",
    description: "The `at-block` function evaluates the expression `expr` _as if_ it were evaluated at the end of the
block indicated by the _block-hash_ argument. The `expr` closure must be read-only.

Note: The block identifying hash must be a hash returned by the `id-header-hash` block information
property. This hash uniquely identifies Stacks blocks and is unique across Stacks forks. While the hash returned by
`header-hash` is unique within the context of a single fork, it is not unique across Stacks forks.

The function returns the result of evaluating `expr`.
",
    example: "(at-block 0x0000000000000000000000000000000000000000000000000000000000000000 (var-get data))
(at-block (get-block-info? id-header-hash (- block-height u10)) (var-get data))"
};


const AS_CONTRACT_API: SpecialAPI = SpecialAPI {
    input_type: "A",
    snippet: "(as-contract ${1:expr})",
    output_type: "A",
    signature: "(as-contract expr)",
    description: "The `as-contract` function switches the current context's `tx-origin` value to the _contract's_
principal and executes `expr` with that context. It returns the resulting value of `expr`.",
    example: "(as-contract (print tx-sender)) ;; Returns 'CTcontract.name"
};

const ASSERTS_API: SpecialAPI = SpecialAPI {
    input_type: "bool, C",
    snippet: "(asserts! ${1:boolean-expr} (err ${2:thrown}))",
    output_type: "bool",
    signature: "(asserts! bool-expr thrown-value)",
    description: "The `asserts!` function admits a boolean argument and asserts its evaluation:
if bool-expr is `true`, `asserts!` returns `true` and proceeds in the program execution.
If the supplied argument is returning a false value, `asserts!` _returns_ `thrown-value` and exits the current
control-flow.",
    example: "(asserts! (is-eq 1 1) (err 1)) ;; Returns true",
};

const EXPECTS_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A) | (response A B), C",
    snippet: "(unwrap! ${1:algebraic-expr} (err ${2:thrown}))",
    output_type: "A",
    signature: "(unwrap! option-input thrown-value)",
    description: "The `unwrap!` function attempts to 'unpack' the first argument: if the argument is
an option type, and the argument is a `(some ...)` option, `unwrap!` returns the inner value of the
option. If the argument is a response type, and the argument is an `(ok ...)` response, `unwrap!` returns
 the inner value of the `ok`. If the supplied argument is either an `(err ...)` or a `(none)` value,
`unwrap!` _returns_ `thrown-value` from the current function and exits the current control-flow.",
    example: "(unwrap! (map-get? names-map (tuple (name \"blockstack\"))) (err 1)) ;; Returns (tuple (id 1337))",
};

const TRY_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A) | (response A B)",
    snippet: "(try! ${1:algebraic-expr})",
    output_type: "A",
    signature: "(try! option-input)",
    description: "The `try!` function attempts to 'unpack' the first argument: if the argument is
an option type, and the argument is a `(some ...)` option, `try!` returns the inner value of the
option. If the argument is a response type, and the argument is an `(ok ...)` response, `try!` returns
 the inner value of the `ok`. If the supplied argument is either an `(err ...)` or a `none` value,
`try!` _returns_ either `none` or the `(err ...)` value from the current function and exits the current control-flow.",
    example: "(try! (map-get? names-map (tuple (name \"blockstack\"))) (err 1)) ;; Returns (tuple (id 1337))
(define-private (checked-even (x int))
  (if (is-eq (mod x 2) 0)
      (ok x)
      (err false)))
(define-private (double-if-even (x int))
  (ok (* 2 (try! (checked-even x)))))
(double-if-even 10) ;; Returns (ok 20)
(double-if-even 3) ;; Returns (err false)
",
};

const UNWRAP_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A) | (response A B)",
    snippet: "(unwrap-panic ${1:algebraic-expr})",
    output_type: "A",
    signature: "(unwrap-panic option-input)",
    description: "The `unwrap` function attempts to 'unpack' its argument: if the argument is
an option type, and the argument is a `(some ...)` option, this function returns the inner value of the
option. If the argument is a response type, and the argument is an `(ok ...)` response, it returns
 the inner value of the `ok`. If the supplied argument is either an `(err ...)` or a `(none)` value,
`unwrap` throws a runtime error, aborting any further processing of the current transaction.",
    example: "(unwrap-panic (map-get? names-map (tuple (name \"blockstack\")))) ;; Returns (tuple (id 1337))",
};

const UNWRAP_ERR_API: SpecialAPI = SpecialAPI {
    input_type: "(response A B)",
    snippet: "(unwrap-err-panic ${1:algebraic-expr})",
    output_type: "B",
    signature: "(unwrap-err-panic response-input)",
    description: "The `unwrap-err` function attempts to 'unpack' the first argument: if the argument
is an `(err ...)` response, `unwrap` returns the inner value of the `err`.
If the supplied argument is an `(ok ...)` value,
`unwrap-err` throws a runtime error, aborting any further processing of the current transaction.",
    example: "(unwrap-err-panic (err 1)) ;; Returns 1",
};

const EXPECTS_ERR_API: SpecialAPI = SpecialAPI {
    input_type: "(response A B), C",
    snippet: "(unwrap-err! ${1:algebraic-expr} (err ${2:thrown}))",
    output_type: "B",
    signature: "(unwrap-err! response-input thrown-value)",
    description: "The `unwrap-err!` function attempts to 'unpack' the first argument: if the argument
is an `(err ...)` response, `unwrap-err!` returns the inner value of the `err`.
If the supplied argument is an `(ok ...)` value,
`unwrap-err!` _returns_ `thrown-value` from the current function and exits the current control-flow.",
    example: "(unwrap-err! (err 1) false) ;; Returns 1",
};

const MATCH_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A) name expression expression | (response A B) name expression name expression",
    snippet: "(match ${1:algebraic-expr} ${2:some-binding-name} ${3:some-branch} ${4:none-branch})",
    output_type: "C",
    signature: "(match opt-input some-binding-name some-branch none-branch) |
(match-resp input ok-binding-name ok-branch err-binding-name err-branch)",
    description: "The `match` function is used to test and destructure optional and response types.

If the `input` is an optional, it tests whether the provided
`input` is a `some` or `none` option, and evaluates `some-branch` or
`none-branch` in each respective case.

Within the `some-branch`, the _contained value_ of the `input`
argument is bound to the provided `some-binding-name` name.

Only _one_ of the branches will be evaluated (similar to `if` statements).

If the `input` is a response, it tests whether the provided `input` is
an `ok` or `err` response type, and evaluates `ok-branch` or
`err-branch` in each respective case.

Within the `ok-branch`, the _contained ok value_ of the `input`
argument is bound to the provided `ok-binding-name` name.

Within the `err-branch`, the _contained err value_ of the `input`
argument is bound to the provided `err-binding-name` name.

Only _one_ of the branches will be evaluated (similar to `if` statements).

Note: Type checking requires that the type of both the ok and err parts of the
response object be determinable. For situations in which one of the parts of a response
is untyped, you should use `unwrap-panic` or `unwrap-err-panic` instead of `match`.",
    example: "
(define-private (add-10 (x (optional int)))
  (match x
  value (+ 10 value)
  10))
(add-10 (some 5)) ;; returns 15
(add-10 none) ;; returns 10

(define-private (add-or-pass-err (x (response int (buff 10))) (to-add int))
  (match x
   value (+ to-add value)
   err-value (err err-value)))
(add-or-pass-err (ok 5) 20) ;; returns 25
(add-or-pass-err (err \"ERROR\") 20) ;; returns (err \"ERROR\")
", };

const DEFAULT_TO_API: SpecialAPI = SpecialAPI {
    input_type: "A, (optional A)",
    snippet: "(default-to ${1:default-value} ${2:option-value})",
    output_type: "A",
    signature: "(default-to default-value option-value)",
    description: "The `default-to` function attempts to 'unpack' the second argument: if the argument is
a `(some ...)` option, it returns the inner value of the option. If the second argument is a `(none)` value,
`default-to` it returns the value of `default-value`.",
    example: "(default-to 0 (get id (map-get? names-map (tuple (name \"blockstack\"))))) ;; Returns 1337
(default-to 0 (get id (map-get? names-map (tuple (name \"non-existant\"))))) ;; Returns 0
",
};

const CONS_OK_API: SpecialAPI = SpecialAPI {
    input_type: "A",
    snippet: "(ok ${1:value})",
    output_type: "(response A B)",
    signature: "(ok value)",
    description: "The `ok` function constructs a response type from the input value. Use `ok` for
creating return values in public functions. An _ok_ value indicates that any database changes during
the processing of the function should materialize.",
    example: "(ok 1) ;; Returns (ok 1)",
};

const CONS_ERR_API: SpecialAPI = SpecialAPI {
    input_type: "A",
    snippet: "(err ${1:value})",
    output_type: "(response A B)",
    signature: "(err value)",
    description: "The `err` function constructs a response type from the input value. Use `err` for
creating return values in public functions. An _err_ value indicates that any database changes during
the processing of the function should be rolled back.",
    example: "(err true) ;; Returns (err true)",
};

const CONS_SOME_API: SpecialAPI = SpecialAPI {
    input_type: "A",
    snippet: "(some ${1:value})",
    output_type: "(optional A)",
    signature: "(some value)",
    description: "The `some` function constructs a `optional` type from the input value.",
    example: "(some 1) ;; Returns (some 1)
(is-none (some 2)) ;; Returns false",
};

const IS_OK_API: SpecialAPI = SpecialAPI {
    input_type: "(response A B)",
    snippet: "(is-ok ${1:expr})",
    output_type: "bool",
    signature: "(is-ok value)",
    description: "`is-ok` tests a supplied response value, returning `true` if the response was `ok`,
and `false` if it was an `err`.",
    example: "(is-ok (ok 1)) ;; Returns true
(is-ok (err 1)) ;; Returns false",
};

const IS_NONE_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A)",
    snippet: "(is-none ${1:expr})",
    output_type: "bool",
    signature: "(is-none value)",
    description: "`is-none` tests a supplied option value, returning `true` if the option value is `(none)`,
and `false` if it is a `(some ...)`.",
    example: "(is-none (get id (map-get? names-map (tuple (name \"blockstack\"))))) ;; Returns false
(is-none (get id (map-get? names-map (tuple (name \"non-existant\"))))) ;; Returns true"
};

const IS_ERR_API: SpecialAPI = SpecialAPI {
    input_type: "(response A B)",
    snippet: "(is-err ${1:expr})",
    output_type: "bool",
    signature: "(is-err? value)",
    description: "`is-err?` tests a supplied response value, returning `true` if the response was an `err`,
and `false` if it was an `ok`.",
    example: "(is-err? (ok 1)) ;; Returns false
(is-err? (err 1)) ;; Returns true",
};

const IS_SOME_API: SpecialAPI = SpecialAPI {
    input_type: "(optional A)",
    snippet: "(is-some ${1:expr})",
    output_type: "bool",
    signature: "(is-some? value)",
    description: "`is-some?` tests a supplied option value, returning `true` if the option value is `(some ...)`,
and `false` if it is a `none`.",
    example: "(is-some? (get id (map-get? names-map (tuple (name \"blockstack\"))))) ;; Returns true
(is-some? (get id (map-get? names-map (tuple (name \"non-existant\"))))) ;; Returns false"
};

const GET_BLOCK_INFO_API: SpecialAPI = SpecialAPI {
    input_type: "BlockInfoPropertyName, BlockHeightInt",
    snippet: "(get-block-info? ${1:prop} ${2:block-height})",
    output_type: "(optional buff) | (optional uint)",
    signature: "(get-block-info? prop-name block-height-expr)",
    description: "The `get-block-info?` function fetches data for a block of the given block height. The
value and type returned are determined by the specified `BlockInfoPropertyName`. If the provided `BlockHeightInt` does
not correspond to an existing block prior to the current block, the function returns `none`. The currently available property names
are `time`, `header-hash`, `burnchain-header-hash`, `id-header-hash`, `miner-address`, and `vrf-seed`.

The `time` property returns an integer value of the block header time field. This is a Unix epoch timestamp in seconds
which roughly corresponds to when the block was mined. **Warning**: this does not increase monotonically with each block
and block times are accurate only to within two hours. See [BIP113](https://github.com/bitcoin/bips/blob/master/bip-0113.mediawiki) for more information.

The `header-hash`, `burnchain-header-hash`, `id-header-hash`, and `vrf-seed` properties return a 32-byte buffer.

The `miner-address` property returns a `principal` corresponding to the miner of the given block.

The `id-header-hash` is the block identifier value that must be used as input to the `at-block` function.
",
    example: "(get-block-info? time u10) ;; Returns (some 1557860301)
(get-block-info? header-hash u2) ;; Returns (some 0x374708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb)
(get-block-info? vrf-seed u6) ;; Returns (some 0xf490de2920c8a35fabeb13208852aa28c76f9be9b03a4dd2b3c075f7a26923b4)
"
};

const DEFINE_TOKEN_API: DefineAPI = DefineAPI {
    input_type: "TokenName, <uint>",
    snippet: "(define-fungible-token ${1:token-name} ${2:total-supply})",
    output_type: "Not Applicable",
    signature: "(define-fungible-token token-name <total-supply>)",
    description: "`define-fungible-token` is used to define a new fungible token class for use in the current contract.

The second argument, if supplied, defines the total supply of the fungible token. This ensures that all calls to the `ft-mint?`
function will never be able to create more than `total-supply` tokens. If any such call were to increase the total supply
of tokens passed that amount, that invocation of `ft-mint?` will result in a runtime error and abort.

Like other kinds of definition statements, `define-fungible-token` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).

Tokens defined using `define-fungible-token` may be used in `ft-transfer?`, `ft-mint?`, and `ft-get-balance` functions",
    example: "
(define-fungible-token stacks)
"
};

const DEFINE_ASSET_API: DefineAPI = DefineAPI {
    input_type: "AssetName, TypeSignature",
    snippet: "(define-non-fungible-token ${1:nft-name} ${2:nft-identifier-type})",
    output_type: "Not Applicable",
    signature: "(define-non-fungible-token asset-name asset-identifier-type)",
    description: "`define-non-fungible-token` is used to define a new non-fungible token class for use in the current contract.
Individual assets are identified by their asset identifier, which must be of the type `asset-identifier-type`. Asset
identifiers are _unique_ identifiers.

Like other kinds of definition statements, `define-non-fungible-token` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).

Assets defined using `define-non-fungible-token` may be used in `nft-transfer?`, `nft-mint?`, and `nft-get-owner?` functions",
    example: "
(define-non-fungible-token names (buff 50))
"
};

const DEFINE_PUBLIC_API: DefineAPI = DefineAPI {
    input_type: "MethodSignature, MethodBody",
    snippet: "(define-public ${1:func-name} ${2:body})",
    output_type: "Not Applicable",
    signature: "(define-public (function-name (arg-name-0 arg-type-0) (arg-name-1 arg-type-1) ...) function-body)",
    description: "`define-public` is used to define a _public_ function and transaction for a smart contract. Public
functions are callable from other smart contracts and may be invoked directly by users by submitting a transaction
to the Stacks blockchain.

Like other kinds of definition statements, `define-public` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).

Public functions _must_ return a ResponseType (using either `ok` or `err`). Any datamap modifications performed by
a public function is aborted if the function returns an `err` type. Public functions may be invoked by other
contracts via `contract-call?`.",
    example: "
(define-public (hello-world (input int))
  (begin (print (+ 2 input))
         (ok input)))
"
};

const DEFINE_CONSTANT_API: DefineAPI = DefineAPI {
    input_type: "MethodSignature, MethodBody",
    snippet: "(define-constant ${1:name} ${2:exprs})",
    output_type: "Not Applicable",
    signature: "(define-constant name expression)",
    description: "`define-constant` is used to define a private constant value in a smart contract.
The expression passed into the definition is evaluated at contract launch, in the order that it is
supplied in the contract. This can lead to undefined function or undefined variable errors in the
event that a function or variable used in the expression has not been defined before the constant.

Like other kinds of definition statements, `define-constant` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).
",
    example: "
(define-constant four (+ 2 2))
(+ 4 four) ;; returns 8
"
};

const DEFINE_PRIVATE_API: DefineAPI = DefineAPI {
    input_type: "MethodSignature, MethodBody",
    snippet: "(define-private ${1:func-name} ${2:body})",
    output_type: "Not Applicable",
    signature: "(define-private (function-name (arg-name-0 arg-type-0) (arg-name-1 arg-type-1) ...) function-body)",
    description: "`define-private` is used to define _private_ functions for a smart contract. Private
functions may not be called from other smart contracts, nor may they be invoked directly by users.
Instead, these functions may only be invoked by other functions defined in the same smart contract.

Like other kinds of definition statements, `define-private` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).

Private functions may return any type.",
    example: "
(define-private (max-of (i1 int) (i2 int))
  (if (> i1 i2)
      i1
      i2))
(max-of 4 6) ;; returns 6
"
};

const DEFINE_READ_ONLY_API: DefineAPI = DefineAPI {
    input_type: "MethodSignature, MethodBody",
    snippet: "(define-read-only ${1:func-name} ${2:body})",
    output_type: "Not Applicable",
    signature: "(define-read-only (function-name (arg-name-0 arg-type-0) (arg-name-1 arg-type-1) ...) function-body)",
    description: "`define-read-only` is used to define a _public read-only_ function for a smart contract. Such
functions are callable from other smart contracts.

Like other kinds of definition statements, `define-read-only` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).

Read-only functions may return any type. However, read-only functions
may not perform any datamap modifications, or call any functions which
perform such modifications. This is enforced both during type checks and during
the execution of the function. Public read-only functions may
be invoked by other contracts via `contract-call?`.",
    example: "
(define-read-only (just-return-one-hundred)
  (* 10 10))"
};

const DEFINE_MAP_API: DefineAPI = DefineAPI {
    input_type: "MapName, KeyTupleDefinition, MapTupleDefinition",
    snippet: "(define-map ${1:map-name} ((${2:key-name-1} ${3:key-type-1})) ((${4:val-name-1} ${5:vals-type-1})))",
    output_type: "Not Applicable",
    signature: "(define-map map-name ((key-name-0 key-type-0) ...) ((val-name-0 val-type-0) ...))",
    description: "`define-map` is used to define a new datamap for use in a smart contract. Such
maps are only modifiable by the current smart contract.

Maps are defined with a key tuple type and value tuple type. These are defined using a list
of name and type pairs, e.g., a key type might be `((id int))`, which is a tuple with a single \"id\"
field of type `int`.

Like other kinds of definition statements, `define-map` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).",
    example: "
(define-map squares ((x int)) ((square int)))
(define-private (add-entry (x int))
  (map-insert squares ((x 2)) ((square (* x x)))))
(add-entry 1)
(add-entry 2)
(add-entry 3)
(add-entry 4)
(add-entry 5)
"
};

const DEFINE_DATA_VAR_API: DefineAPI = DefineAPI {
    input_type: "VarName, TypeDefinition, Value",
    snippet: "(define-data-var ${1:name} ${2:type} ${3:value})",
    output_type: "Not Applicable",
    signature: "(define-data-var var-name type value)",
    description: "`define-data-var` is used to define a new persisted variable for use in a smart contract. Such
variable are only modifiable by the current smart contract.

Persisted variable are defined with a type and a value.

Like other kinds of definition statements, `define-data-var` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).",
    example: "
(define-data-var size int 0)
(define-private (set-size (value int))
  (var-set size value))
(set-size 1)
(set-size 2)
"
};

const DEFINE_TRAIT_API: DefineAPI = DefineAPI {
    input_type: "VarName, [MethodSignature]",
    snippet: "(define-trait ${1:trait-name} ${2:trait-definition})",
    output_type: "Not Applicable",
    signature: "(define-trait trait-name ((func1-name (arg1-type arg2-type ...) (return-type))))",
    description: "`define-trait` is used to define a new trait definition for use in a smart contract. Other contracts
can implement a given trait and then have their contract identifier being passed as function arguments in order to be called
dynamically with `contract-call?`.

Traits are defined with a name, and a list functions defined with a name, a list of argument types, and return type.

Like other kinds of definition statements, `define-trait` may only be used at the top level of a smart contract
definition (i.e., you cannot put a define statement in the middle of a function body).
",
    example: "
(define-trait token-trait
    ((transfer? (principal principal uint) (response uint uint))
     (get-balance (principal) (response uint uint))))
"
};

const USE_TRAIT_API: DefineAPI = DefineAPI {
    input_type: "VarName, TraitIdentifier",
    snippet: "(use-trait ${1:trait-alias} ${2:trait-identifier})",
    output_type: "Not Applicable",
    signature: "(use-trait trait-alias trait-identifier)",
    description: "`use-trait` is used to bring a trait, defined in another contract, to the current contract. Subsequent
references to an imported trait are signaled with the syntax <trait-alias>.

Traits import are defined with a name, used as an alias, and a trait identifier. Trait identifiers can either be
using the sugared syntax (.token-a.token-trait), or be fully qualified ('SPAXYA5XS51713FDTQ8H94EJ4V579CXMTRNBZKSF.token-a.token-trait).

Like other kinds of definition statements, `use-trait` may only be used at the top level of a smart contract
definition (i.e., you cannot put such a statement in the middle of a function body).
    ",
    example: "
(use-trait token-a-trait 'SPAXYA5XS51713FDTQ8H94EJ4V579CXMTRNBZKSF.token-a.token-trait)
(define-public (forward-get-balance (user principal) (contract <token-a-trait>))
  (begin
    (ok 1)))
"
};

const IMPL_TRAIT_API: DefineAPI = DefineAPI {
    input_type: "TraitIdentifier",
    snippet: "(impl-trait ${2:trait-identifier})",
    output_type: "Not Applicable",
    signature: "(impl-trait trait-identifier)",
    description: "`impl-trait` can be use for asserting that a contract is fully implementing a given trait.
Additional checks are being performed when the contract is being published, rejecting the deployment if the
contract is violating the trait specification.

Trait identifiers can either be using the sugared syntax (.token-a.token-trait), or be fully qualified
('SPAXYA5XS51713FDTQ8H94EJ4V579CXMTRNBZKSF.token-a.token-trait).

Like other kinds of definition statements, `impl-trait` may only be used at the top level of a smart contract
definition (i.e., you cannot put such a statement in the middle of a function body).
",
    example: "
(impl-trait 'SPAXYA5XS51713FDTQ8H94EJ4V579CXMTRNBZKSF.token-a.token-trait)
"
};

const MINT_TOKEN: SpecialAPI = SpecialAPI {
    input_type: "TokenName, uint, principal",
    snippet: "(ft-mint? ${1:token-name} ${2:amount} ${3:recipient})",
    output_type: "(response bool uint)",
    signature: "(ft-mint? token-name amount recipient)",
    description: "`ft-mint?` is used to increase the token balance for the `recipient` principal for a token
type defined using `define-fungible-token`. The increased token balance is _not_ transfered from another principal, but
rather minted.

If a non-positive amount is provided to mint, this function returns `(err 1)`. Otherwise, on successfuly mint, it
returns `(ok true 1)`.
",
    example: "
(define-fungible-token stackaroo)
(ft-mint? stackaroo u100 tx-sender)
"
};

const MINT_ASSET: SpecialAPI = SpecialAPI {
    input_type: "AssetName, A, principal",
    snippet: "(nft-mint? ${1:asset-name} ${2:asset-identifier} ${3:recipient})",
    output_type: "(response bool uint)",
    signature: "(nft-mint? asset-class asset-identifier recipient)",
    description: "`nft-mint?` is used to instantiate an asset and set that asset's owner to the `recipient` principal.
The asset must have been defined using `define-non-fungible-token`, and the supplied `asset-identifier` must be of the same type specified in
that definition.

If an asset identified by `asset-identifier` _already exists_, this function will return an error with the following error code:

`(err 1)`

Otherwise, on successfuly mint, it returns `(ok true 1)`.
",
    example: "
(define-non-fungible-token stackaroo (buff 40))
(nft-mint? stackaroo \"Roo\" tx-sender)
"
};

const GET_OWNER: SpecialAPI = SpecialAPI {
    input_type: "AssetName, A",
    snippet: "(nft-get-owner? ${1:asset-name} ${2:asset-identifier})",
    output_type: "(optional principal)",
    signature: "(nft-get-owner? asset-class asset-identifier)",
    description: "`nft-get-owner?` returns the owner of an asset, identified by `asset-identifier`, or `none` if the asset does not exist.
The asset type must have been defined using `define-non-fungible-token`, and the supplied `asset-identifier` must be of the same type specified in
that definition.",
    example: "
(define-non-fungible-token stackaroo (buff 40))
(nft-get-owner? stackaroo \"Roo\")
"
};


const GET_BALANCE: SpecialAPI = SpecialAPI {
    input_type: "TokenName, principal",
    snippet: "(ft-get-balance ${1:token-name} ${2:principal})",
    output_type: "uint",
    signature: "(ft-get-balance token-name principal)",
    description: "`ft-get-balance` returns `token-name` balance of the principal `principal`.
The token type must have been defined using `define-fungible-token`.",
    example: "
(define-fungible-token stackaroos)
(ft-get-balance stackaroos tx-sender)
"
};

const TOKEN_TRANSFER: SpecialAPI = SpecialAPI {
    input_type: "TokenName, uint, principal, principal",
    snippet: "(ft-transfer? ${1:token-name} ${2:amount} ${3:sender} ${4:recipient})",
    output_type: "(response bool uint)",
    signature: "(ft-transfer? token-name amount sender recipient)",
    description: "`ft-transfer?` is used to increase the token balance for the `recipient` principal for a token
type defined using `define-fungible-token` by debiting the `sender` principal.

This function returns (ok true) if the transfer is successful. In the event of an unsuccessful transfer it returns
one of the following error codes:

`(err u1)` -- `sender` does not have enough balance to transfer
`(err u2)` -- `sender` and `recipient` are the same principal
`(err u3)` -- amount to send is non-positive
",
    example: "
(define-fungible-token stackaroo)
(ft-mint? stackaroo u100 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR)
(ft-transfer? stackaroo u50 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (ok true)
(ft-transfer? stackaroo u60 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (err u1)
"
};

const ASSET_TRANSFER: SpecialAPI = SpecialAPI {
    input_type: "AssetName, A, principal, principal",
    snippet: "(nft-transfer? ${1:asset-name} ${2:asset-identifier} ${3:sender} ${4:recipient})",
    output_type: "(response bool uint)",
    signature: "(nft-transfer? asset-class asset-identifier sender recipient)",
    description: "`nft-transfer?` is used to change the owner of an asset identified by `asset-identifier`
from `sender` to `recipient`. The `asset-class` must have been defined by `define-non-fungible-token` and `asset-identifier`
must be of the type specified in that definition.

This function returns (ok true) if the transfer is successful. In the event of an unsuccessful transfer it returns
one of the following error codes:

`(err u1)` -- `sender` does not own the asset
`(err u2)` -- `sender` and `recipient` are the same principal
`(err u3)` -- asset identified by asset-identifier does not exist
",
    example: "
(define-non-fungible-token stackaroo (buff 40))
(nft-mint? stackaroo \"Roo\" 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR)
(nft-transfer? stackaroo \"Roo\" 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (ok true)
(nft-transfer? stackaroo \"Roo\" 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (err u1)
(nft-transfer? stackaroo \"Stacka\" 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (err u3)
"
};

const STX_TRANSFER: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(stx-transfer? ${1:amount} ${2:sender} ${3:recipient})",
    signature: "(stx-transfer? amount sender recipient)",
    description: "`stx-transfer?` is used to increase the STX balance for the `recipient` principal
by debiting the `sender` principal. The `sender` principal _must_ be equal to the current context's `tx-sender`.

This function returns (ok true) if the transfer is successful. In the event of an unsuccessful transfer it returns
one of the following error codes:

`(err u1)` -- `sender` does not have enough balance to transfer
`(err u2)` -- `sender` and `recipient` are the same principal
`(err u3)` -- amount to send is non-positive
`(err u4)` -- the `sender` principal is not the current `tx-sender`
",
    example: "
(stx-transfer? u50 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR tx-sender) ;; returns (err u4)
(stx-transfer? u60 tx-sender 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR) ;; returns (ok true)
"
};

const STX_BURN: SimpleFunctionAPI = SimpleFunctionAPI {
    name: None,
    snippet: "(stx-burn? ${1:amount} ${2:sender})",
    signature: "(stx-burn? amount sender)",
    description: "`stx-burn?` debits the `sender` principal's STX holdings by `amount`, destroying
the STX. The `sender` principal _must_ be equal to the current context's `tx-sender`.

This function returns (ok true) if the transfer is successful. In the event of an unsuccessful transfer it returns
one of the following error codes:

`(err u1)` -- `sender` does not have enough balance to transfer
`(err u3)` -- amount to send is non-positive
`(err u4)` -- the `sender` principal is not the current `tx-sender`
",
    example: "
(stx-burn? u50 'SZ2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKQ9H6DPR) ;; returns (err u4)
(stx-burn? u60 tx-sender) ;; returns (ok true)
"
};

pub fn make_api_reference(function: &NativeFunctions) -> FunctionAPI {
    use crate::clarity::functions::NativeFunctions::*;
    let name = function.get_name();
    match function {
        Add => make_for_simple_native(&ADD_API, &Add, name),
        ToUInt => make_for_simple_native(&TO_UINT_API, &ToUInt, name),
        ToInt => make_for_simple_native(&TO_INT_API, &ToInt, name),
        Subtract => make_for_simple_native(&SUB_API, &Subtract, name),
        Multiply => make_for_simple_native(&MUL_API, &Multiply, name),
        Divide => make_for_simple_native(&DIV_API, &Divide, name),
        CmpGeq => make_for_simple_native(&GEQ_API, &CmpGeq, name),
        CmpLeq => make_for_simple_native(&LEQ_API, &CmpLeq, name),
        CmpLess => make_for_simple_native(&LESS_API, &CmpLess, name),
        CmpGreater => make_for_simple_native(&GREATER_API, &CmpGreater, name),
        Modulo => make_for_simple_native(&MOD_API, &Modulo, name),
        Power => make_for_simple_native(&POW_API, &Power, name),
        BitwiseXOR => make_for_simple_native(&XOR_API, &BitwiseXOR, name),
        And => make_for_simple_native(&AND_API, &And, name),
        Or => make_for_simple_native(&OR_API, &Or, name),
        Not => make_for_simple_native(&NOT_API, &Not, name),
        Equals => make_for_special(&EQUALS_API, name),
        If => make_for_special(&IF_API, name),
        Let => make_for_special(&LET_API, name),
        FetchVar => make_for_special(&FETCH_VAR_API, name),
        SetVar => make_for_special(&SET_VAR_API, name),
        Map => make_for_special(&MAP_API, name),
        Filter => make_for_special(&FILTER_API, name),
        Fold => make_for_special(&FOLD_API, name),
        Append => make_for_special(&APPEND_API, name),
        Concat => make_for_special(&CONCAT_API, name),
        AsMaxLen => make_for_special(&ASSERTS_MAX_LEN_API, name),
        Len => make_for_special(&LEN_API, name),
        ListCons => make_for_special(&LIST_API, name),
        FetchEntry => make_for_special(&FETCH_ENTRY_API, name),
        SetEntry => make_for_special(&SET_ENTRY_API, name),
        InsertEntry => make_for_special(&INSERT_ENTRY_API, name),
        DeleteEntry => make_for_special(&DELETE_ENTRY_API, name),
        TupleCons => make_for_special(&TUPLE_CONS_API, name),
        TupleGet => make_for_special(&TUPLE_GET_API, name),
        Begin => make_for_special(&BEGIN_API, name),
        Hash160 => make_for_special(&HASH160_API, name),
        Sha256 => make_for_special(&SHA256_API, name),
        Sha512 => make_for_special(&SHA512_API, name),
        Sha512Trunc256 => make_for_special(&SHA512T256_API, name),
        Keccak256 => make_for_special(&KECCAK256_API, name),
        Print => make_for_special(&PRINT_API, name),
        ContractCall => make_for_special(&CONTRACT_CALL_API, name),
        AsContract => make_for_special(&AS_CONTRACT_API, name),
        GetBlockInfo => make_for_special(&GET_BLOCK_INFO_API, name),
        ConsOkay => make_for_special(&CONS_OK_API, name),
        ConsError => make_for_special(&CONS_ERR_API, name),
        ConsSome =>  make_for_special(&CONS_SOME_API, name),
        DefaultTo => make_for_special(&DEFAULT_TO_API, name),
        Asserts => make_for_special(&ASSERTS_API, name),
        UnwrapRet => make_for_special(&EXPECTS_API, name),
        UnwrapErrRet => make_for_special(&EXPECTS_ERR_API, name),
        Unwrap => make_for_special(&UNWRAP_API, name),
        UnwrapErr => make_for_special(&UNWRAP_ERR_API, name),
        Match => make_for_special(&MATCH_API, name),
        TryRet =>  make_for_special(&TRY_API, name),
        IsOkay => make_for_special(&IS_OK_API, name),
        IsNone => make_for_special(&IS_NONE_API, name),
        IsErr => make_for_special(&IS_ERR_API, name),
        IsSome => make_for_special(&IS_SOME_API, name),
        MintAsset => make_for_special(&MINT_ASSET, name),
        MintToken => make_for_special(&MINT_TOKEN, name),
        GetTokenBalance => make_for_special(&GET_BALANCE, name),
        GetAssetOwner => make_for_special(&GET_OWNER, name),
        TransferToken => make_for_special(&TOKEN_TRANSFER, name),
        TransferAsset => make_for_special(&ASSET_TRANSFER, name),
        AtBlock => make_for_special(&AT_BLOCK, name),
        StxTransfer => make_for_simple_native(&STX_TRANSFER, &StxTransfer, name),
        StxBurn => make_for_simple_native(&STX_BURN, &StxBurn, name),
    }
}

pub fn make_keyword_reference(variable: &NativeVariables) -> KeywordAPI {
    match variable {
        NativeVariables::TxSender => TX_SENDER_KEYWORD.clone(),
        NativeVariables::ContractCaller => CONTRACT_CALLER_KEYWORD.clone(),
        NativeVariables::NativeNone => NONE_KEYWORD.clone(),
        NativeVariables::NativeTrue => TRUE_KEYWORD.clone(),
        NativeVariables::NativeFalse => FALSE_KEYWORD.clone(),
        NativeVariables::BlockHeight => BLOCK_HEIGHT.clone(),
        NativeVariables::BurnBlockHeight => BURN_BLOCK_HEIGHT.clone(),
    }
}

fn make_for_special(api: &SpecialAPI, name: String) -> FunctionAPI {
    FunctionAPI {
        name,
        snippet: api.snippet.to_string(),
        input_type: api.input_type.to_string(),
        output_type: api.output_type.to_string(),
        signature: api.signature.to_string(),
        description: api.description.to_string(),
        example: api.example.to_string()
    }
}

pub fn make_for_define(api: &DefineAPI, name: String) -> FunctionAPI {
    FunctionAPI {
        name,
        snippet: api.snippet.to_string(),
        input_type: api.input_type.to_string(),
        output_type: api.output_type.to_string(),
        signature: api.signature.to_string(),
        description: api.description.to_string(),
        example: api.example.to_string()
    }
}

pub fn make_define_reference(define_type: &DefineFunctions) -> FunctionAPI {
    let name = define_type.get_name();
    match &*define_type.get_name() {
        "define-constant" => make_for_define(&DEFINE_CONSTANT_API, name),
        "define-private" => make_for_define(&DEFINE_PRIVATE_API, name),
        "define-public" => make_for_define(&DEFINE_PUBLIC_API, name),
        "define-read-only" => make_for_define(&DEFINE_READ_ONLY_API, name),
        "define-map" =>  make_for_define(&DEFINE_MAP_API, name),
        "define-data-var" => make_for_define(&DEFINE_DATA_VAR_API, name),
        "define-fungible-token" => make_for_define(&DEFINE_TOKEN_API, name),
        "define-non-fungible-token" => make_for_define(&DEFINE_ASSET_API, name),
        "define-trait" => make_for_define(&DEFINE_TRAIT_API, name),
        "use-trait" => make_for_define(&USE_TRAIT_API, name),
        "impl-trait" => make_for_define(&IMPL_TRAIT_API, name),
        _ => unreachable!()
    }
}

pub fn make_json_api_reference() -> String {
    let mut functions: Vec<_> = NativeFunctions::ALL.iter()
        .map(|x| make_api_reference(x))
        .collect();

    for data_type in DefineFunctions::ALL.iter() {
        functions.push(make_define_reference(data_type))
    }

    let mut keywords = Vec::new();
    for variable in NativeVariables::ALL.iter() {
        let api_ref = make_keyword_reference(variable);
        keywords.push(api_ref);
    }

    let api_out = ReferenceAPIs { functions, keywords };
    format!("{}", serde_json::to_string(&api_out)
            .expect("Failed to serialize documentation"))
}
