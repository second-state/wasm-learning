#include <wasmedge/wasmedge.h>
#include <wasmedge/wasmedge-image.h>
#include <wasmedge/wasmedge-tensorflowlite.h>

#include <stdio.h>

int main(int argc, char *argv[]) {
  /*
   * argv[0]: ./a.out
   * argv[1]: WASM file
   * argv[2]: tflite model file
   * argv[3]: image file
   * Usage: ./a.out birds_v1.wasm lite-model_aiy_vision_classifier_birds_V1_3.tflite bird.jpg
   */

  /* Create the VM context. */
  WasmEdge_ConfigureContext *ConfCxt = WasmEdge_ConfigureCreate();
  WasmEdge_ConfigureAddHostRegistration(ConfCxt, WasmEdge_HostRegistration_Wasi);
  WasmEdge_VMContext *VMCxt = WasmEdge_VMCreate(ConfCxt, NULL);
  WasmEdge_ConfigureDelete(ConfCxt);
  
  /* Create the image and TFLite import objects. */
  WasmEdge_ImportObjectContext *ImageImpObj = WasmEdge_Image_ImportObjectCreate();
  WasmEdge_ImportObjectContext *TFLiteImpObj = WasmEdge_TensorflowLite_ImportObjectCreate();
  WasmEdge_ImportObjectContext *TFDummyImpObj = WasmEdge_Tensorflow_ImportObjectCreateDummy();

  /* Register into VM. */
  WasmEdge_VMRegisterModuleFromImport(VMCxt, ImageImpObj);
  WasmEdge_VMRegisterModuleFromImport(VMCxt, TFLiteImpObj);
  WasmEdge_VMRegisterModuleFromImport(VMCxt, TFDummyImpObj);

  /* Init WASI. */
  const char *Preopens[] = {".:."};
  const char *Args[] = {argv[1], argv[2], argv[3]};
  WasmEdge_ImportObjectContext *WASIImpObj = WasmEdge_VMGetImportModuleContext(VMCxt, WasmEdge_HostRegistration_Wasi);
  WasmEdge_ImportObjectInitWASI(WASIImpObj, Args, 3, NULL, 0, Preopens, 1);

  /* Run WASM file. */
  WasmEdge_String FuncName = WasmEdge_StringCreateByCString("_start");
  WasmEdge_Result Res = WasmEdge_VMRunWasmFromFile(VMCxt, argv[1], FuncName, NULL, 0, NULL, 0);
  WasmEdge_StringDelete(FuncName);

  /* Check the result. */
  if (!WasmEdge_ResultOK(Res)) {
    printf("Run WASM failed: %s\n", WasmEdge_ResultGetMessage(Res));
    return -1;
  }

  WasmEdge_ImportObjectDelete(ImageImpObj);
  WasmEdge_ImportObjectDelete(TFLiteImpObj);
  WasmEdge_ImportObjectDelete(TFDummyImpObj);
  WasmEdge_VMDelete(VMCxt);
  return 0;
}
