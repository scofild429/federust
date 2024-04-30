# Function
This chapter only describes the function of learning part. They are all marked to the corresponding part in archtechture graph.

## A: Send request to initialize ONNX
As a project is going to be build, we send this require to server module, ask for initial ONNX file.

## B: Initialization of trusted ONNX in Server module
Even in some projects only Client module is deployed, the trusted ONNX part will still be initialized in Server module, but only this part. To initialize the trusted ONNX part, multiple source options are allowed, such as from ONNX Zoo, or from some exposed trusted ONNX part in other project.

## C: Assigne trusted ONNX to Init ONNX
Pull the newest trusted ONNX file from Server module to Init. ONNX in Client module. This step can be used as the response of function <span style="color:blue">*A*</span>, also can be used for updating Init ONNX file for next training.

## D: Training model with local data
lauching model from ONNX, and training with local data. After each training task, our model will be saved as updated ONNX, 

## E: Init ONNX for application
If the Init ONNX is good enough, or project is not going to train any local data, we can launch the model from Init ONNX for Application.

## F: Updated ONNX for application
When the model has been trained with local data, it should have different performance for application.

## J: Continual training
If our model performs not enough good, we train it again and again, and replace updated ONNX in place.

## G: Push updated ONNX to server module
According to the project function, we are able to push the updated ONNX to ONNX pool in server module. 

## K: ONNX pool function
The ONNX pool of each project can only contain certainly number of ONNX files. They can be aggreated, combined. For flexiblity, We hope multiple language can be supported, such as Python.

## H: Update trusted ONNX
with aggreatation, combination we can update the trusted ONNX in server module. Before replacing the trusted ONNX, we can use validation dataset to confirm the performance of our new model.


## I: Drop ONNX from ONNX pool
Validation dataset can be used to select some bad or poisoning ONNX and drop them.



