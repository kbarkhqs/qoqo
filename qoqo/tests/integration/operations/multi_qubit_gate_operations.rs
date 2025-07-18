// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use ndarray::Array2;
use num_complex::Complex64;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;
use pyo3::Python;
use qoqo::operations::convert_operation_to_pyobject;
#[cfg(feature = "unstable_operation_definition")]
use qoqo::operations::CallDefinedGateWrapper;
use qoqo::operations::{
    MultiQubitCNOTWrapper, MultiQubitMSWrapper, MultiQubitZZWrapper, QFTWrapper,
};
use qoqo::CircuitWrapper;
use qoqo_calculator::Calculator;
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use roqoqo::operations::Operation;
use roqoqo::operations::*;
#[cfg(feature = "json_schema")]
use roqoqo::ROQOQO_VERSION;
use roqoqo::{Circuit, RoqoqoError};
use std::convert::TryInto;
use std::{
    collections::HashMap,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use test_case::test_case;

use super::convert_cf_to_pyobject;

#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__eq__"; "MultiQubitMS_eq")]
#[test_case(Operation::from(MultiQubitMS::new(vec![2, 3], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__ne__"; "MultiQubitMS_ne")]
fn test_new_multi_qubit_ms(input_operation: Operation, arguments: (Vec<u32>, f64), method: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // Basic initialisation, no errors
        let operation_type = py.get_type::<MultiQubitMSWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<MultiQubitMSWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1(([0, 1], vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<MultiQubitMSWrapper>().unwrap();
        let binding = operation_type.call1((vec![1, 2], 0.0)).unwrap();
        let new_op_diff = binding.downcast::<MultiQubitMSWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<MultiQubitMSWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "MultiQubitMSWrapper { internal: MultiQubitMS { qubits: [1, 2], theta: Float(0.0) } }"
        );
    })
}

#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__eq__"; "MultiQubitZZ_eq")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![2, 3], CalculatorFloat::ZERO)), (vec![0, 1], 0.0,), "__ne__"; "MultiQubitZZ_ne")]
fn test_new_multi_qubit_zz(input_operation: Operation, arguments: (Vec<u32>, f64), method: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // Basic initialisation, no errors
        let operation_type = py.get_type::<MultiQubitZZWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<MultiQubitZZWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1(([0, 1], vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<MultiQubitZZWrapper>().unwrap();
        let binding = operation_type.call1((vec![1, 2], 0.0)).unwrap();
        let new_op_diff = binding.downcast::<MultiQubitZZWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<MultiQubitZZWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "MultiQubitZZWrapper { internal: MultiQubitZZ { qubits: [1, 2], theta: Float(0.0) } }"
        );
    })
}

#[cfg(feature = "unstable_operation_definition")]
#[test_case(Operation::from(CallDefinedGate::new("name".to_owned(), vec![0, 1], vec![CalculatorFloat::from(0.0)])), ("name".to_owned(), vec![0, 1], vec![0.0],), "__eq__"; "CallDefinedGate_eq")]
#[test_case(Operation::from(CallDefinedGate::new("name".to_owned(), vec![2, 3], vec![CalculatorFloat::from(0.0)])), ("name".to_owned(), vec![0, 1], vec![0.0],), "__ne__"; "CallDefinedGate_ne")]
fn test_new_call_defined_gate(
    input_operation: Operation,
    arguments: (String, Vec<u32>, Vec<f64>),
    method: &str,
) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // Basic initialisation, no errors
        let operation_type = py.get_type::<CallDefinedGateWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<CallDefinedGateWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1(([0, 1], vec!["fails"]));
        assert!(result.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<CallDefinedGateWrapper>().unwrap();
        let new_op_diff = operation_type
            .call1(("name".to_owned(), vec![1, 2], vec![0.0]))
            .unwrap();
        let def_wrapper_diff = new_op_diff
            .downcast::<CallDefinedGateWrapper>()
            .unwrap()
            .extract::<CallDefinedGateWrapper>()
            .unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "CallDefinedGateWrapper { internal: CallDefinedGate { gate_name: \"name\", qubits: [1, 2], free_parameters: [Float(0.0)] } }"
        );
    })
}

#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1])), vec![0, 1], "__eq__"; "MultiQubitCNOT_eq")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![2, 3])), vec![0, 1], "__ne__"; "MultiQubitCNOT_ne")]
fn test_new_multi_cnot(input_operation: Operation, arguments: Vec<u32>, method: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // Basic initialisation, no errors
        let operation_type = py.get_type::<MultiQubitCNOTWrapper>();
        let binding = operation_type.call1((arguments,)).unwrap();
        let operation_py = binding.downcast::<MultiQubitCNOTWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((vec!["fails"],));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<MultiQubitCNOTWrapper>().unwrap();
        let binding = operation_type.call1((vec![1, 2],)).unwrap();
        let new_op_diff = binding.downcast::<MultiQubitCNOTWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<MultiQubitCNOTWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "MultiQubitCNOTWrapper { internal: MultiQubitCNOT { qubits: [1, 2] } }"
        );
    })
}

#[test_case(Operation::from(QFT::new(vec![0, 1], true, true)), (vec![0, 1], true, true), "__eq__"; "QFT_eq")]
#[test_case(Operation::from(QFT::new(vec![2, 3], true, true)), (vec![0, 1], true, true), "__ne__"; "QFT_ne")]
fn test_new_qft(input_operation: Operation, arguments: (Vec<u32>, bool, bool), method: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // Basic initialisation, no errors
        let operation_type = py.get_type::<QFTWrapper>();
        let binding = operation_type.call1(arguments).unwrap();
        let operation_py = binding.downcast::<QFTWrapper>().unwrap();
        let comparison =
            bool::extract_bound(&operation.call_method1(method, (operation_py,)).unwrap()).unwrap();
        assert!(comparison);

        // Error initialisation
        let result = operation_type.call1((vec!["fails"], false, false));
        let result_ref = result.as_ref();
        assert!(result_ref.is_err());

        // Testing PartialEq, Clone and Debug
        let def_wrapper = operation_py.extract::<QFTWrapper>().unwrap();
        let binding = operation_type.call1((vec![1, 2], true, true)).unwrap();
        let new_op_diff = binding.downcast::<QFTWrapper>().unwrap();
        let def_wrapper_diff = new_op_diff.extract::<QFTWrapper>().unwrap();
        let helper_ne: bool = def_wrapper_diff != def_wrapper;
        assert!(helper_ne);
        let helper_eq: bool = def_wrapper == def_wrapper.clone();
        assert!(helper_eq);

        assert_eq!(
            format!("{def_wrapper_diff:?}"),
            "QFTWrapper { internal: QFT { qubits: [1, 2], swaps: true, inverse: true } }"
        );

        // Testing inputs
        let swaps_input = bool::extract_bound(&operation.call_method0("swaps").unwrap()).unwrap();
        assert!(swaps_input);
        let inverse_input =
            bool::extract_bound(&operation.call_method0("inverse").unwrap()).unwrap();
        assert!(inverse_input);
    })
}

/// Test is_parametrized() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitZZ")]
fn test_pyo3_is_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        assert!(operation
            .call_method0("is_parametrized")
            .unwrap()
            .extract::<bool>()
            .unwrap());
    })
}

/// Test is_parametrized = false for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::PI)); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::PI)); "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1])); "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1], true, true)); "QFT")]
fn test_pyo3_is_not_parametrized(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        assert!(!operation
            .call_method0("is_parametrized")
            .unwrap()
            .extract::<bool>()
            .unwrap());
    })
}

/// Test theta() function for Rotations
#[test_case(CalculatorFloat::from(0), Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS float")]
#[test_case(CalculatorFloat::from("theta"), Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitMS symb")]
#[test_case(CalculatorFloat::from(0), Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitZZ float")]
#[test_case(CalculatorFloat::from("theta"), Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from("theta"))); "MultiQubitZZ symb")]
fn test_pyo3_theta(theta: CalculatorFloat, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let theta_op: CalculatorFloatWrapper =
            operation.call_method0("theta").unwrap().extract().unwrap();
        let theta_param: CalculatorFloatWrapper =
            CalculatorFloatWrapper::extract_bound(&convert_cf_to_pyobject(py, theta)).unwrap();
        assert_eq!(theta_op.internal, theta_param.internal);
    })
}

/// Test qubits() function for MultiQubitGate Operations
#[test_case(vec![0, 1], Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS two")]
#[test_case(vec![0, 1, 2], Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))); "MultiQubitMS three")]
#[test_case(vec![0, 1], Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitZZ two")]
#[test_case(vec![0, 1, 2], Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0))); "MultiQubitZZ three")]
#[test_case(vec![0], Operation::from(MultiQubitCNOT::new(vec![0])); "MultiQubitCNOT one")]
#[test_case(vec![0, 1], Operation::from(MultiQubitCNOT::new(vec![0, 1])); "MultiQubitCNOT two")]
#[test_case(vec![0, 1, 2], Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])); "MultiQubitCNOT three")]
#[test_case(vec![0], Operation::from(QFT::new(vec![0], true, true)); "QFT one")]
#[test_case(vec![0, 1], Operation::from(QFT::new(vec![0, 1], true, true)); "QFT two")]
#[test_case(vec![0, 1, 2], Operation::from(QFT::new(vec![0, 1, 2], true, true)); "QFT three")]
fn test_pyo3_qubits(qubit: Vec<usize>, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let qubit_op: Vec<usize> = operation.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubit_op, qubit);
    })
}

/// Test hqslang() function for MultiQubitGate Operations
#[test_case("MultiQubitMS", Operation::from(MultiQubitMS::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitMS")]
#[test_case("MultiQubitZZ", Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from(0))); "MultiQubitZZ")]
#[test_case("MultiQubitCNOT", Operation::from(MultiQubitCNOT::new(vec![0, 1])); "MultiQubitCNOT")]
#[test_case("QFT", Operation::from(QFT::new(vec![0, 1], true, false)); "QFT")]
fn test_pyo3_hqslang(name: &'static str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let name_op: String = operation
            .call_method0("hqslang")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(name_op, name.to_string());
    })
}

/// Test tags() function for MultiQubitGate Operations
#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "MultiQubitGateOperation",
        "MultiQubitMS",
        ];
    "MultiQubitMS")]
#[test_case(
    Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    vec![
        "Operation",
        "GateOperation",
        "MultiQubitGateOperation",
        "MultiQubitZZ",
        ];
    "MultiQubitZZ")]
#[test_case(
    Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])),
    vec![
        "Operation",
        "GateOperation",
        "MultiQubitGateOperation",
        "MultiQubitCNOT",
        ];
    "MultiQubitCNOT")]
#[test_case(
    Operation::from(QFT::new(vec![0, 1, 2], false, true)),
    vec![
        "Operation",
        "GateOperation",
        "MultiQubitGateOperation",
        "QFT",
        ];
    "QFT")]
fn test_pyo3_tags(input_operation: Operation, tags: Vec<&str>) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let tags_op: Vec<String> = operation.call_method0("tags").unwrap().extract().unwrap();
        assert_eq!(tags_op.len(), tags.len());
        for i in 0..tags.len() {
            assert_eq!(tags_op[i], tags[i]);
        }
    })
}

// Test CallDefinedGate's tags, hslang and is_parametrized functions
#[cfg(feature = "unstable_operation_definition")]
#[test_case(Operation::from(CallDefinedGate::new("name".to_owned(), vec![0, 1], vec![CalculatorFloat::from(0.0)])); "CallDefinedGate")]
fn test_pyo3_gate_definition(input_definition: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_definition, py).unwrap();

        let to_tag = operation.call_method0("tags").unwrap();
        let tags_op: &Vec<String> = &to_tag.extract().unwrap();
        let tags_param: &[&str] = &["Operation", "MultiQubitGateOperation", "CallDefinedGate"];
        assert_eq!(tags_op, tags_param);

        let hqslang_op: String = operation
            .call_method0("hqslang")
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(hqslang_op, "CallDefinedGate");

        assert!(!operation
            .call_method0("is_parametrized")
            .unwrap()
            .extract::<bool>()
            .unwrap());
    })
}

/// Test inputs for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_call_defined_gate_inputs() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();

        // Test gate_name()
        let name_op: String = operation
            .call_method0("gate_name")
            .unwrap()
            .extract()
            .unwrap();
        let name_param: String = String::from("name");
        assert_eq!(name_op, name_param);

        // Test qubits()
        let qubits: Vec<usize> = operation.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubits, vec![1, 2]);

        // Test free_parameters()
        let mut free_parameters: Vec<CalculatorFloat> = vec![];
        let py_params = operation.call_method0("free_parameters").unwrap();
        let params: Bound<pyo3::types::PyList> = py_params.extract().unwrap();
        for param in pyo3::types::PyListMethods::iter(&params) {
            free_parameters.push(
                qoqo_calculator_pyo3::convert_into_calculator_float(&param.as_borrowed()).unwrap(),
            );
        }
        assert_eq!(free_parameters, vec![CalculatorFloat::from(0.0)]);
    })
}

/// Test remap_qubits() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])); "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1, 2], true, true)); "QFT")]
fn test_pyo3_remapqubits(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // test initial qubit
        let qubits: Vec<usize> = operation.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubits, vec![0, 1, 2]);
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 2);
        qubit_mapping.insert(2, 0);
        let result = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        // test re-mapped qubit
        let qubits_new: Vec<usize> = result.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubits_new, vec![1, 2, 0]);
        // test that initial and rempapped qubits are different
        assert_ne!(qubits, qubits_new);
    })
}

/// Test remap_qubits() function for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_remapqubits_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![0, 1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();
        // test initial qubit
        let qubits: Vec<usize> = operation.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubits, vec![0, 1, 2]);
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(0, 1);
        qubit_mapping.insert(1, 2);
        qubit_mapping.insert(2, 0);
        let result = operation
            .call_method1("remap_qubits", (qubit_mapping,))
            .unwrap();
        // test re-mapped qubit
        let qubits_new: Vec<usize> = result.call_method0("qubits").unwrap().extract().unwrap();
        assert_eq!(qubits_new, vec![1, 2, 0]);
        // test that initial and rempapped qubits are different
        assert_ne!(qubits, qubits_new);
    })
}

// test remap_qubits() function returning an error.
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])); "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1, 2], false, false)); "QFT")]
fn test_pyo3_remapqubits_error(input_operation: Operation) {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1("remap_qubits", (qubit_mapping,));
        assert!(result.is_err());
    })
}

/// test remap_qubits() function returning an error.
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_remapqubits_error_call_defined_gate() {
    // preparation
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();
        // remap qubits
        let mut qubit_mapping: HashMap<usize, usize> = HashMap::new();
        qubit_mapping.insert(2, 0);
        let result = operation.call_method1("remap_qubits", (qubit_mapping,));
        assert!(result.is_err());
    })
}

/// Test unitary_matrix() function for MultiQubitGate Operations
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])); "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1, 2], true, false)); "QFT")]
fn test_pyo3_unitarymatrix(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let py_result = operation.call_method0("unitary_matrix").unwrap();
        let result_matrix: Array2<Complex64> = py_result
            .extract::<PyReadonlyArray2<Complex64>>()
            .unwrap()
            .as_array()
            .to_owned();

        // compare to reference matrix obtained in Rust directly (without passing to Python)
        let gate: MultiQubitGateOperation = input_operation.try_into().unwrap();
        let rust_matrix: Result<Array2<Complex64>, RoqoqoError> = gate.unitary_matrix();
        let test_matrix: Array2<Complex64> = rust_matrix.unwrap();

        assert_eq!(result_matrix, test_matrix);
    })
}

/// Test unitary_matrix() function for MultiQubitGate Operations for the error case
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from("PI"))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from("PI"))); "MultiQubitZZ")]
fn test_pyo3_unitarymatrix_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let py_result = operation.call_method0("unitary_matrix");
        assert!(py_result.is_err());
    })
}

/// Test circuit() function for MultiQubitMS
#[test]
fn test_pyo3_circuit_ms() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation =
            Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.0)));
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let py_result = operation.call_method0("circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract().unwrap();

        let mut circuit = Circuit::new();
        circuit += Hadamard::new(0);
        circuit += Hadamard::new(1);
        circuit += Hadamard::new(2);
        circuit += CNOT::new(0, 1);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, CalculatorFloat::from(1.0));
        circuit += CNOT::new(1, 2);
        circuit += CNOT::new(0, 1);
        circuit += Hadamard::new(0);
        circuit += Hadamard::new(1);
        circuit += Hadamard::new(2);

        assert_eq!(result_circuit.internal, circuit);
    })
}

/// Test circuit() function for MultiQubitZZ
#[test]
fn test_pyo3_circuit_zz() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation =
            Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(1.0)));
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let py_result = operation.call_method0("circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract().unwrap();

        let mut circuit = Circuit::new();
        circuit += CNOT::new(0, 1);
        circuit += CNOT::new(1, 2);
        circuit += RotateZ::new(2, CalculatorFloat::from(1.0));
        circuit += CNOT::new(1, 2);
        circuit += CNOT::new(0, 1);

        assert_eq!(result_circuit.internal, circuit);
    })
}

/// Test circuit() function for MultiQubitCNOT
#[test]
fn test_pyo3_circuit_multi_cnot() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(MultiQubitCNOT::new(vec![0, 1, 2]));
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let py_result = operation.call_method0("circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract().unwrap();

        let mut circuit = Circuit::new();
        circuit += Hadamard::new(2);
        circuit += CNOT::new(1, 2);
        circuit += PhaseShiftState1::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(2);
        circuit += CNOT::new(1, 2);
        circuit += PhaseShiftState1::new(2, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 2);
        circuit += TGate::new(1);
        circuit += TGate::new(2);
        circuit += Hadamard::new(2);
        circuit += CNOT::new(0, 1);
        circuit += TGate::new(0);
        circuit += PhaseShiftState1::new(1, -CalculatorFloat::FRAC_PI_4);
        circuit += CNOT::new(0, 1);

        assert_eq!(result_circuit.internal, circuit);
    })
}

/// Test circuit() function for QFT
#[test]
fn test_pyo3_circuit_qft() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let input_operation = Operation::from(QFT::new(vec![0, 1, 2], false, false));
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let py_result = operation.call_method0("circuit").unwrap();
        let result_circuit: CircuitWrapper = py_result.extract().unwrap();

        let mut circuit = Circuit::new();
        circuit += Hadamard::new(0);
        circuit += ControlledPhaseShift::new(1, 0, FRAC_PI_2.into());
        circuit += ControlledPhaseShift::new(2, 0, FRAC_PI_4.into());
        circuit += Hadamard::new(1);
        circuit += ControlledPhaseShift::new(2, 1, FRAC_PI_2.into());
        circuit += Hadamard::new(2);

        assert_eq!(result_circuit.internal, circuit);
    })
}

/// Test copy and deepcopy functions
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(1.3))); "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])); "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1, 2], true, true)); "QFT")]
fn test_pyo3_copy_deepcopy(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let copy_op = operation.call_method0("__copy__").unwrap();
        let deepcopy_op = operation.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .call_method1("__eq__", (copy_deepcopy_param.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test copy and deepcopy functions for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_copy_deepcopy_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();
        let copy_op = operation.call_method0("__copy__").unwrap();
        let deepcopy_op = operation.call_method1("__deepcopy__", ("",)).unwrap();
        let copy_deepcopy_param = operation;

        let comparison_copy = bool::extract_bound(
            &copy_op
                .call_method1("__eq__", (copy_deepcopy_param.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_copy);
        let comparison_deepcopy = bool::extract_bound(
            &deepcopy_op
                .call_method1("__eq__", (copy_deepcopy_param,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison_deepcopy);
    })
}

/// Test format and repr functions
#[test_case(
    "MultiQubitMS { qubits: [0, 1, 2], theta: Float(0.0) }",
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::ZERO));
    "MultiQubitMS")]
#[test_case(
    "MultiQubitZZ { qubits: [0, 1, 2], theta: Float(0.0) }",
    Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::ZERO));
    "MultiQubitZZ")]
#[test_case(
    "MultiQubitCNOT { qubits: [0, 1, 2] }",
    Operation::from(MultiQubitCNOT::new(vec![0, 1, 2]));
    "MultiQubitCNOT")]
#[test_case(
    "QFT { qubits: [0, 1, 2], swaps: false, inverse: false }",
    Operation::from(QFT::new(vec![0, 1, 2], false, false));
    "QFT")]
fn test_pyo3_format_repr(format_repr: &str, input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let to_format = operation.call_method1("__format__", ("",)).unwrap();
        let format_op: String = to_format.extract().unwrap();
        assert_eq!(format_op, format_repr);
        let to_repr = operation.call_method0("__repr__").unwrap();
        let repr_op: String = to_repr.extract().unwrap();
        assert_eq!(repr_op, format_repr);
    })
}

/// Test format and repr functions for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_format_repr_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();
        let to_format = operation.call_method1("__format__", ("",)).unwrap();
        let format_op: String = to_format.extract().unwrap();
        assert_eq!(
            format_op,
            "CallDefinedGate { gate_name: \"name\", qubits: [1, 2], free_parameters: [Float(0.0)] }"
        );
        let to_repr = operation.call_method0("__repr__").unwrap();
        let repr_op: String = to_repr.extract().unwrap();
        assert_eq!(
            repr_op,
            "CallDefinedGate { gate_name: \"name\", qubits: [1, 2], free_parameters: [Float(0.0)] }"
        );
    })
}

/// Test substitute_parameters() function for one parameter
#[test_case(Operation::from(MultiQubitMS::new(vec![1, 2, 3], CalculatorFloat::from("theta"))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![1, 2, 3], CalculatorFloat::from("theta"))); "MultiQubitZZ")]
fn test_pyo3_substitute_params_rotate(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation.clone(), py).unwrap();
        let mut substitution_dict_py: HashMap<String, f64> = HashMap::new();
        substitution_dict_py.insert("theta".to_owned(), 1.0);
        let substitute_op = operation
            .call_method1("substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = input_operation
            .substitute_parameters(&substitution_dict)
            .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param, py).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_substitute_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from("theta")],
            )),
            py,
        )
        .unwrap();
        let mut substitution_dict_py: HashMap<&str, f64> = HashMap::new();
        substitution_dict_py.insert("theta", 1.0);
        let substitute_op = operation
            .call_method1("substitute_parameters", (substitution_dict_py,))
            .unwrap();

        let mut substitution_dict: Calculator = Calculator::new();
        substitution_dict.set_variable("theta", 1.0);
        let substitute_param = Operation::from(CallDefinedGate::new(
            "name".to_owned(),
            vec![1, 2],
            vec![CalculatorFloat::from("theta")],
        ))
        .substitute_parameters(&substitution_dict)
        .unwrap();
        let test_operation = convert_operation_to_pyobject(substitute_param, py).unwrap();

        let comparison = bool::extract_bound(
            &substitute_op
                .call_method1("__eq__", (test_operation,))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);
    })
}

/// Test substitute_parameters() causing an error `None`
#[test_case(Operation::from(MultiQubitMS::new(vec![1, 2], CalculatorFloat::from("test"))); "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![1, 2], CalculatorFloat::from("test"))); "MultiQubitZZ")]
fn test_pyo3_substitute_params_error(input_operation: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(input_operation, py).unwrap();
        let substitution_dict: HashMap<String, f64> = HashMap::new();
        let result = operation.call_method1("substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_substitute_params_error_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from("test")],
            )),
            py,
        )
        .unwrap();
        let substitution_dict: HashMap<&str, f64> = HashMap::new();
        let result = operation.call_method1("substitute_parameters", (substitution_dict,));
        assert!(result.is_err());
    })
}

#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0.005))),
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0.005 * 1.5))); "MultiQubitMS")]
#[test_case(
    Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0.005))),
    Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0.005 * 1.5))); "MultiQubitZZ")]
fn test_pyo3_rotate_powercf(first_op: Operation, second_op: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation = convert_operation_to_pyobject(first_op, py).unwrap();

        let power = convert_cf_to_pyobject(py, CalculatorFloat::from(1.5));
        let comparison_op = convert_operation_to_pyobject(second_op, py).unwrap();

        let remapped_op = operation.call_method1("powercf", (power,)).unwrap();
        let comparison = remapped_op
            .call_method1("__eq__", (comparison_op,))
            .unwrap()
            .extract::<bool>()
            .unwrap();
        assert!(comparison);
    })
}

/// Test the __richcmp__ function
#[test_case(
    Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    Operation::from(MultiQubitMS::new(vec![1, 2], CalculatorFloat::from(0))); "MultiQubitMS")]
#[test_case(
    Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0))),
    Operation::from(MultiQubitZZ::new(vec![1, 2], CalculatorFloat::from(0))); "MultiQubitZZ")]
#[test_case(
    Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])),
    Operation::from(MultiQubitCNOT::new(vec![1, 2])); "MultiQubitCNOT")]
#[test_case(
    Operation::from(QFT::new(vec![0, 1, 2], false, false)),
    Operation::from(QFT::new(vec![0, 1, 2], true, false)); "QFT")]
fn test_pyo3_richcmp(definition_1: Operation, definition_2: Operation) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(definition_1, py).unwrap();
        let operation_two = convert_operation_to_pyobject(definition_2, py).unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1("__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1("__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

/// Test the __richcmp__ function for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[test]
fn test_pyo3_richcmp_call_defined_gate() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let operation_one = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![0, 1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();
        let operation_two = convert_operation_to_pyobject(
            Operation::from(CallDefinedGate::new(
                "name".to_owned(),
                vec![1, 2],
                vec![CalculatorFloat::from(0.0)],
            )),
            py,
        )
        .unwrap();

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__eq__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(!comparison);

        let comparison = bool::extract_bound(
            &operation_one
                .call_method1("__ne__", (operation_two.clone(),))
                .unwrap(),
        )
        .unwrap();
        assert!(comparison);

        let comparison = operation_one.call_method1("__eq__", (vec!["fails"],));
        assert!(comparison.is_err());

        let comparison = operation_one.call_method1("__ge__", (operation_two,));
        assert!(comparison.is_err());
    })
}

#[cfg(feature = "json_schema")]
#[test_case(Operation::from(MultiQubitMS::new(vec![0, 1, 2], CalculatorFloat::from(0))), "1.0.0"; "MultiQubitMS")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1, 2], CalculatorFloat::from(0))), "1.0.0"; "MultiQubitZZ")]
#[test_case(Operation::from(MultiQubitCNOT::new(vec![0, 1, 2])), "1.20.0"; "MultiQubitCNOT")]
#[test_case(Operation::from(QFT::new(vec![0, 1, 2], true, true)), "1.20.0"; "QFT")]
fn test_pyo3_json_schema(operation: Operation, version_string: &str) {
    let rust_schema = match operation {
        Operation::MultiQubitMS(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(MultiQubitMS)).unwrap()
        }
        Operation::MultiQubitZZ(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(MultiQubitZZ)).unwrap()
        }
        Operation::MultiQubitCNOT(_) => {
            serde_json::to_string_pretty(&schemars::schema_for!(MultiQubitCNOT)).unwrap()
        }
        Operation::QFT(_) => serde_json::to_string_pretty(&schemars::schema_for!(QFT)).unwrap(),
        _ => unreachable!(),
    };
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pyobject = convert_operation_to_pyobject(operation, py).unwrap();
        let operation = pyobject;

        let schema: String =
            String::extract_bound(&operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&operation.call_method0("min_supported_version").unwrap())
                .unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, version_string);
    });
}

/// Test the json schema for CallDefinedGate
#[cfg(feature = "unstable_operation_definition")]
#[cfg(feature = "json_schema")]
#[test]
fn test_pyo3_json_schema_call_defined_gate() {
    let operation = Operation::from(CallDefinedGate::new(
        "name".to_owned(),
        vec![1, 2],
        vec![CalculatorFloat::from(0.0)],
    ));
    let rust_schema =
        serde_json::to_string_pretty(&schemars::schema_for!(CallDefinedGate)).unwrap();
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        let pyobject = convert_operation_to_pyobject(operation, py).unwrap();
        let operation = pyobject;

        let schema: String =
            String::extract_bound(&operation.call_method0("json_schema").unwrap()).unwrap();

        assert_eq!(schema, rust_schema);

        let current_version_string =
            String::extract_bound(&operation.call_method0("current_version").unwrap()).unwrap();
        let minimum_supported_version_string =
            String::extract_bound(&operation.call_method0("min_supported_version").unwrap())
                .unwrap();

        assert_eq!(current_version_string, ROQOQO_VERSION);
        assert_eq!(minimum_supported_version_string, "1.13.0");
    });
}
