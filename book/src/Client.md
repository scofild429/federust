# Client module
## Auth part
1. A1 -> create user instance, save in postgresql
2. B1 -> user authentication, verify JWT with data  from postgresql
3. C1 -> create a project in Client module
  1. Send request to Agency module for download ONNX file as initialization state.
  2. Save  ONNX file? seeing above ONNX chapter
  3. Create project with ONNX file
  4. Assignment the authorization to project
  5. Return created project  information
4. C2 -> create the project again for current user with given project information
  1. In Client module, different user in the same project do exactly the same thing
  2. All user can push their updated ONNX(after training) to Agency module
  
## learning part
Will be implemented in program languages, such as python, Golang, C... . Those program languages can by called by rust with Foreign Function Interface.

- D1 -> training the model with local data
  - Can choose prefer language and AI framework
  - Stream output display to user in Interface module
  - Specify the path of local data
  - if replace the ONNX file in place
- E1 -> push the updated ONNX file in Client module to Agency module.

