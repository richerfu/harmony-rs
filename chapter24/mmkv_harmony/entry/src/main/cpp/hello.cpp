#include "napi/native_api.h"
#include "MMKV.h"
#include <string>

static napi_value MMKVInit(napi_env env, napi_callback_info info) {
    std::string dir = "/data/storage/el2/base/haps/entry/files/mmkv";
    MMKV::initializeMMKV(dir);
    auto mmkv = MMKV::defaultMMKV();
    mmkv->set(3.14f, "float");
    float a = mmkv->getFloat("float");
    return nullptr;
}

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports) {
    napi_property_descriptor desc[] = {{"mmkvInit", nullptr, MMKVInit, nullptr, nullptr, nullptr, napi_default, nullptr}};
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
extern "C" __attribute__((constructor)) void RegisterEntryModule(void) { napi_module_register(&demoModule); }