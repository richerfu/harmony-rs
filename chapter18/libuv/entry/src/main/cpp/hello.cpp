#include "napi/native_api.h"

struct CallbackData {
    napi_async_work asyncWork = nullptr;
    napi_deferred deferred = nullptr;
    double args = 0;
    double result = 0;
};

double FibonacciRecursive(double n) {
    if (n <= 1)
        return n;
    return FibonacciRecursive(n - 1) + FibonacciRecursive(n - 2);
}

static void ExecuteCB(napi_env env, void *data) {
    CallbackData *callbackData = reinterpret_cast<CallbackData *>(data);
    callbackData->result = FibonacciRecursive(callbackData->args);
}

static void CompleteCB(napi_env env, napi_status status, void *data) {
    CallbackData *callbackData = reinterpret_cast<CallbackData *>(data);
    napi_value result = nullptr;
    napi_create_double(env, callbackData->result, &result);
    if (callbackData->result > 0) {
        napi_resolve_deferred(env, callbackData->deferred, result);
    } else {
        napi_reject_deferred(env, callbackData->deferred, result);
    }

    napi_delete_async_work(env, callbackData->asyncWork);
    delete callbackData;
}

static napi_value AsyncWork(napi_env env, napi_callback_info info) {
    size_t argc = 1;
    napi_value args[1];
    napi_get_cb_info(env, info, &argc, args, nullptr, nullptr);

    napi_value promise = nullptr;
    napi_deferred deferred = nullptr;
    napi_create_promise(env, &deferred, &promise);

    auto callbackData = new CallbackData();
    callbackData->deferred = deferred;
    napi_get_value_double(env, args[0], &callbackData->args);

    napi_value resourceName = nullptr;
    napi_create_string_utf8(env, "AsyncCallback", NAPI_AUTO_LENGTH, &resourceName);
    // 创建异步任务
    napi_create_async_work(env, nullptr, resourceName, ExecuteCB, CompleteCB, callbackData, &callbackData->asyncWork);
    // 将异步任务加入队列
    napi_queue_async_work(env, callbackData->asyncWork);

    return promise;
}

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc[] = {
        {"asyncWork", nullptr, AsyncWork, nullptr, nullptr, nullptr, napi_default, nullptr}};
    napi_define_properties(env, exports, sizeof(desc) / sizeof(desc[0]), desc);
    return exports;
}
EXTERN_C_END

static napi_module demoModule = {
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "entry",
    .nm_priv = ((void *)0),
    .reserved = {0},
};

extern "C" __attribute__((constructor)) void RegisterEntryModule(void) {
    napi_module_register(&demoModule);
}
