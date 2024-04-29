# Server module

this module will be wrote totally in Rust

- F1 -> set the initialization of trusted ONNX
  - This use case is only for when user only deploys Agency module, without Client module.
  - and the user is going to offer this trusted ONNX file in Agency module for other users.

- F2 -> set the ONNX pool configuration for selection algorithmus
  - Under which condition we can replace trusted ONNX in a project
  - Under which condition we should drop ONNX file in ONNX pool

- F3 ->  run the ONNX pool selection process on Validation data
  - Validation data is independent from training data
  - After Validation, we have to update the trusted ONNX or drop the ONNX file in ONNX, so that ONNX pool has limited number of ONNX file.
